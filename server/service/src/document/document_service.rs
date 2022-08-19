use jsonschema::JSONSchema;
use repository::{
    Document, DocumentFilter, DocumentRepository, FormSchemaRowRepository, RepositoryError,
    StorageConnection,
};

use crate::service_provider::ServiceContext;

use super::raw_document::RawDocument;

#[derive(Debug, PartialEq)]
pub enum DocumentInsertError {
    InvalidParent(String),
    /// Input document doesn't match the provided json schema
    InvalidDataSchema(Vec<String>),
    DataSchemaDoesNotExist,
    DatabaseError(RepositoryError),
    InternalError(String),
}

#[derive(Debug)]
pub enum DocumentHistoryError {
    DatabaseError(RepositoryError),
    InternalError(String),
}

impl From<RepositoryError> for DocumentHistoryError {
    fn from(err: RepositoryError) -> Self {
        DocumentHistoryError::DatabaseError(err)
    }
}

pub trait DocumentServiceTrait: Sync + Send {
    fn get_document(
        &self,
        ctx: &ServiceContext,
        name: &str,
    ) -> Result<Option<Document>, RepositoryError> {
        DocumentRepository::new(&ctx.connection).find_one_by_name(name)
    }

    fn get_documents(
        &self,
        ctx: &ServiceContext,
        filter: Option<DocumentFilter>,
    ) -> Result<Vec<Document>, RepositoryError> {
        DocumentRepository::new(&ctx.connection).query(filter)
    }

    fn get_document_history(
        &self,
        ctx: &ServiceContext,
        name: &str,
    ) -> Result<Vec<Document>, DocumentHistoryError> {
        let repo = DocumentRepository::new(&ctx.connection);
        let docs = repo.document_history(name)?;
        Ok(docs)
    }

    fn update_document(
        &self,
        ctx: &ServiceContext,
        doc: RawDocument,
    ) -> Result<Document, DocumentInsertError> {
        let document = ctx
            .connection
            .transaction_sync(|con| {
                let validator = json_validator(con, &doc)?;
                if let Some(validator) = &validator {
                    validate_json(&validator, &doc.data)
                        .map_err(|errors| DocumentInsertError::InvalidDataSchema(errors))?;
                }
                if let Some(invalid_parent) = validate_parents(con, &doc)? {
                    return Err(DocumentInsertError::InvalidParent(invalid_parent));
                }

                insert_document(con, doc)
            })
            .map_err(|err| err.to_inner_error())?;
        Ok(document)
    }
}

pub struct DocumentService {}
impl DocumentServiceTrait for DocumentService {}

impl From<RepositoryError> for DocumentInsertError {
    fn from(err: RepositoryError) -> Self {
        DocumentInsertError::DatabaseError(err)
    }
}

fn json_validator(
    connection: &StorageConnection,
    doc: &RawDocument,
) -> Result<Option<JSONSchema>, DocumentInsertError> {
    let schema_id = match &doc.schema_id {
        Some(schema_id) => schema_id,
        None => return Ok(None),
    };

    let schema_repo = FormSchemaRowRepository::new(connection);
    let schema = schema_repo
        .find_one_by_id(&schema_id)?
        .ok_or(DocumentInsertError::DataSchemaDoesNotExist)?;
    let compiled = match JSONSchema::compile(&schema.json_schema) {
        Ok(v) => Ok(v),
        Err(err) => Err(DocumentInsertError::InternalError(format!(
            "Invalid json schema: {}",
            err
        ))),
    }?;
    Ok(Some(compiled))
}

fn validate_json(validator: &JSONSchema, data: &serde_json::Value) -> Result<(), Vec<String>> {
    validator.validate(data).map_err(|errors| {
        let errors: Vec<String> = errors.map(|err| format!("{}", err)).collect();
        errors
    })
}

// Returns Some invalid parent or None
fn validate_parents(
    connection: &StorageConnection,
    doc: &RawDocument,
) -> Result<Option<String>, RepositoryError> {
    let repo = DocumentRepository::new(connection);
    for parent in &doc.parents {
        if repo.find_one_by_id(&parent)?.is_none() {
            return Ok(Some(parent.clone()));
        }
    }
    Ok(None)
}

/// Does a raw insert without schema validation
fn insert_document(
    connection: &StorageConnection,
    doc: RawDocument,
) -> Result<Document, DocumentInsertError> {
    let doc = doc
        .finalise()
        .map_err(|err| DocumentInsertError::InternalError(err))?;
    let repo = DocumentRepository::new(connection);
    repo.insert(&doc)?;
    Ok(doc)
}

#[cfg(test)]
mod document_service_test {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use repository::{
        mock::{mock_form_schema_empty, mock_form_schema_simple, MockDataInserts},
        test_db::setup_all,
    };
    use serde_json::json;

    use crate::{document::raw_document::RawDocument, service_provider::ServiceProvider};

    use super::*;

    #[actix_rt::test]
    async fn test_document_updates() {
        let (_, _, connection_manager, _) = setup_all(
            "test_document_updates",
            MockDataInserts::none().form_schemas(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let context = service_provider.context().unwrap();
        let service = service_provider.document_service;

        let doc_name = "test/doc2";
        // successfully insert a document
        let v1 = service
            .update_document(
                &context,
                RawDocument {
                    name: doc_name.to_string(),
                    parents: vec![],
                    author: "me".to_string(),
                    timestamp: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(5000, 0),
                        Utc,
                    ),
                    r#type: "test_data".to_string(),
                    data: json!({
                      "version": 1,
                    }),
                    schema_id: None,
                },
            )
            .unwrap();
        let found = service.get_document(&context, doc_name).unwrap().unwrap();
        assert_eq!(found, v1);

        // invalid parents
        let result = service.update_document(
            &context,
            RawDocument {
                name: doc_name.to_string(),
                parents: vec!["invalid".to_string()],
                author: "me".to_string(),
                timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(6000, 0), Utc),
                r#type: "test_data".to_string(),
                data: json!({
                  "version": 2,
                }),
                schema_id: None,
            },
        );
        assert!(matches!(result, Err(DocumentInsertError::InvalidParent(_))));

        // successfully update a document
        let v2 = service
            .update_document(
                &context,
                RawDocument {
                    name: doc_name.to_string(),
                    parents: vec![v1.id.clone()],
                    author: "me".to_string(),
                    timestamp: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(6000, 0),
                        Utc,
                    ),
                    r#type: "test_data".to_string(),
                    data: json!({
                      "version": 2,
                    }),
                    schema_id: None,
                },
            )
            .unwrap();
        assert_eq!(v2.parent_ids[0], v1.id);
        let found = service.get_document(&context, doc_name).unwrap().unwrap();
        assert_eq!(found, v2);
        assert_eq!(found.data["version"], 2);

        // add some noise
        service
            .update_document(
                &context,
                RawDocument {
                    name: "test/noise".to_string(),
                    parents: vec![],
                    author: "me".to_string(),
                    timestamp: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(8000, 0),
                        Utc,
                    ),
                    r#type: "test_data2".to_string(),
                    data: json!({
                      "version": 1,
                    }),
                    schema_id: None,
                },
            )
            .unwrap();
        // should still find the correct document
        let found = service.get_document(&context, doc_name).unwrap().unwrap();
        assert_eq!(found.id, v2.id);
    }

    #[actix_rt::test]
    async fn test_document_schema_validation() {
        let (_, _, connection_manager, _) = setup_all(
            "document_schema_validation",
            MockDataInserts::none().form_schemas(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let context = service_provider.context().unwrap();

        let service = service_provider.document_service;

        // empty schema accepts all data
        let schema = mock_form_schema_empty();
        service
            .update_document(
                &context,
                RawDocument {
                    name: "test/doc1".to_string(),
                    parents: vec![],
                    author: "me".to_string(),
                    timestamp: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(5000, 0),
                        Utc,
                    ),
                    r#type: "test_data".to_string(),
                    data: json!({
                      "value1": "base",
                      "map": {},
                    }),
                    schema_id: Some(schema.id),
                },
            )
            .unwrap();

        // fails with invalid schema
        let schema = mock_form_schema_simple();
        let result = service.update_document(
            &context,
            RawDocument {
                name: "test/doc2".to_string(),
                parents: vec![],
                author: "me".to_string(),
                timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(5000, 0), Utc),
                r#type: "test_data".to_string(),
                data: json!({
                  "value1": "base",
                  "map": {},
                }),
                schema_id: Some(schema.id),
            },
        );
        assert!(matches!(
            result,
            Err(DocumentInsertError::InvalidDataSchema(_))
        ));

        // fails with schema type mismatch
        let schema = mock_form_schema_simple();
        let result = service.update_document(
            &context,
            RawDocument {
                name: "test/doc3".to_string(),
                parents: vec![],
                author: "me".to_string(),
                timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(5000, 0), Utc),
                r#type: "test_data".to_string(),
                data: json!({
                  "intValue": "base",
                  "strValue": 9,
                }),
                schema_id: Some(schema.id),
            },
        );
        assert!(matches!(
            result,
            Err(DocumentInsertError::InvalidDataSchema(_))
        ));

        // succeeds with valid schema
        let schema = mock_form_schema_simple();
        service
            .update_document(
                &context,
                RawDocument {
                    name: "test/doc4".to_string(),
                    parents: vec![],
                    author: "me".to_string(),
                    timestamp: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(5000, 0),
                        Utc,
                    ),
                    r#type: "test_data".to_string(),
                    data: json!({
                      "intValue": 3,
                      "strValue": "str",
                    }),
                    schema_id: Some(schema.id),
                },
            )
            .unwrap();
    }
}
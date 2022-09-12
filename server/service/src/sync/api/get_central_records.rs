use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, PartialEq, Debug, Serialize)]

pub(crate) struct CentralSyncRecordV5 {
    #[serde(rename = "ID")]
    pub(crate) id: u32,
    #[serde(flatten)]
    pub(crate) record: CommonSyncRecordV5,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct CentralSyncBatchV5 {
    #[serde(rename = "maxCursor")]
    pub(crate) max_cursor: u32,
    #[serde(default)]
    pub(crate) data: Vec<CentralSyncRecordV5>,
}

impl SyncApiV5 {
    // Pull batch of records from central sync log.
    pub(crate) async fn get_central_records(
        &self,
        cursor: u32,
        limit: u32,
    ) -> Result<CentralSyncBatchV5, SyncApiError> {
        // TODO: add constants for query parameters.
        let query = [
            ("cursor", &cursor.to_string()),
            ("limit", &limit.to_string()),
        ];
        let response = self.do_get("/sync/v5/central_records", &query).await?;

        to_json(response)
            .await
            .map_err(SyncApiError::ResponseParsingError)
    }
}

#[cfg(test)]
mod test {
    use httpmock::{Method::GET, MockServer};
    use serde_json::json;
    use util::assert_matches;

    use super::*;
    #[actix_rt::test]
    async fn test_get_central_records() {
        let mock_server = MockServer::start();
        let url = mock_server.base_url();

        let mock = mock_server.mock(|when, then| {
            when.method(GET)
                .query_param("cursor", "100")
                .query_param("limit", "2")
                .path("/sync/v5/central_records");
            then.status(200).body(
                r#"{
                "maxCursor": 200,
                "data": [
                    {
                        "ID": 2,
                        "tableName": "test_table_1",
                        "recordId": "ID2",
                        "action": "delete"
                    },
                    {
                        "ID": 3,
                        "tableName": "test_table_2",
                        "recordId": "ID4",
                        "action": "insert",
                        "recordData": {
                            "test_key": "test_value"
                        }
                    }
                ]
            }"#,
            );
        });

        let result = create_api(&url, "", "").get_central_records(100, 2).await;

        mock.assert();

        assert_matches!(result, Ok(_));

        assert_eq!(
            result.unwrap(),
            CentralSyncBatchV5 {
                max_cursor: 200,
                data: vec![
                    CentralSyncRecordV5 {
                        id: 2,
                        record: CommonSyncRecordV5 {
                            table_name: "test_table_1".to_string(),
                            record_id: "ID2".to_string(),
                            action: SyncActionV5::Delete,
                            data: json!({})
                        }
                    },
                    CentralSyncRecordV5 {
                        id: 3,
                        record: CommonSyncRecordV5 {
                            table_name: "test_table_2".to_string(),
                            record_id: "ID4".to_string(),
                            action: SyncActionV5::Insert,
                            data: json!({
                                "test_key": "test_value"
                            })
                        }
                    }
                ]
            }
        );
    }
}

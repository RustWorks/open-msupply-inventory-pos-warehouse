use std::sync::Arc;

use crate::{
    document::patient::{PatientService, PatientServiceTrait, UpdatePatient, PATIENT_TYPE},
    service_provider::ServiceProvider,
};
use repository::{
    DocumentContext, DocumentRegistryRow, DocumentRegistryRowRepository, EqualFilter, FormSchema,
    FormSchemaRowRepository, RepositoryError, StoreFilter, StoreRepository,
};
use serde::{Deserialize, Serialize};
use util::uuid::uuid;

schemafy::schemafy!("src/sync/program_schemas/patient.json");

const PATIENT_SCHEMA: &'static str = std::include_str!("./program_schemas/patient.json");
const PATIENT_UI_SCHEMA: &'static str =
    std::include_str!("./program_schemas/patient_ui_schema.json");

const PROGRAM_SCHEMA: &'static str = std::include_str!("./program_schemas/program.json");
const PROGRAM_UI_SCHEMA: &'static str =
    std::include_str!("./program_schemas/program_ui_schema.json");

const ENCOUNTER_SCHEMA: &'static str = std::include_str!("./program_schemas/encounter.json");
const ENCOUNTER_UI_SCHEMA: &'static str =
    std::include_str!("./program_schemas/encounter_ui_schema.json");

fn person_1() -> Person {
    Person {
        id: Some("person1".to_string()),
        national_id: Some("id34568".to_string()),
        first_name: Some("Tom".to_string()),
        last_name: Some("Smith".to_string()),
        addresses: vec![],
        contact_details: vec![],
        date_of_birth: None,
        date_of_birth_is_estimated: None,
        gender: Some(Gender::Male),
        passport_number: None,
        socio_economics: SocioEconomics {
            education: None,
            literate: None,
            occupation: None,
        },
    }
}

fn person_2() -> Person {
    Person {
        id: Some("person2".to_string()),
        national_id: Some("id41325".to_string()),
        first_name: Some("Eli".to_string()),
        last_name: Some("Bond".to_string()),
        addresses: vec![],
        contact_details: vec![],
        date_of_birth: None,
        date_of_birth_is_estimated: None,
        gender: Some(Gender::Female),
        passport_number: None,
        socio_economics: SocioEconomics {
            education: None,
            literate: None,
            occupation: None,
        },
    }
}

fn person_3() -> Person {
    Person {
        id: Some("person3".to_string()),
        national_id: Some("id12245".to_string()),
        first_name: Some("Heidi".to_string()),
        last_name: Some("Tomalla".to_string()),
        addresses: vec![],
        contact_details: vec![],
        date_of_birth: None,
        date_of_birth_is_estimated: None,
        gender: Some(Gender::Female),
        passport_number: None,
        socio_economics: SocioEconomics {
            education: None,
            literate: None,
            occupation: None,
        },
    }
}

fn patient_1() -> Patient {
    let address = Address {
        key: "home".to_string(),
        description: Some("Home address".to_string()),
        address_1: Some("Anzac Av 1".to_string()),
        address_2: Some("1055 Auckland".to_string()),
        city: None,
        country: Some("NZ".to_string()),
        district: Some("Auckland".to_string()),
        region: None,
        zip_code: None,
    };
    let contact_details = ContactDetails {
        key: "work".to_string(),
        description: Some("Work contact".to_string()),
        mobile: Some("022235678".to_string()),
        phone: Some("095425378".to_string()),
        email: Some("myemail@work.com".to_string()),
        website: Some("mywebsite.com".to_string()),
    };
    Patient {
        id: "patient1".to_string(),
        national_id: Some("id12345".to_string()),
        addresses: vec![address.clone()],
        contact_details: vec![contact_details.clone()],
        date_of_birth: Some("2000-03-04".to_string()),
        date_of_birth_is_estimated: None,
        family: Some(Family {
            marital_status: Some(MaritalStatus::Married),
            caregiver: Some(person_1()),
            mother: Some(person_2()),
            next_of_kin: Some(person_3()),
        }),
        first_name: Some("Tina".to_string()),
        last_name: Some("Ling".to_string()),
        gender: Some(Gender::Female),
        health_center: None,
        passport_number: None,
        socio_economics: SocioEconomics {
            education: None,
            literate: None,
            occupation: None,
        },
        birthorder: None,
        hand: None,
    }
}

fn patient_2() -> Patient {
    let address = Address {
        key: "home".to_string(),
        description: Some("Home address".to_string()),
        address_1: Some("Queen St 55".to_string()),
        address_2: Some("1052 Auckland".to_string()),
        city: None,
        country: Some("NZ".to_string()),
        district: Some("Auckland".to_string()),
        region: None,
        zip_code: None,
    };
    let contact_details = ContactDetails {
        description: None,
        email: Some("cook@mail.com".to_string()),
        key: "private".to_string(),
        mobile: Some("021245678".to_string()),
        phone: Some("092425678".to_string()),
        website: Some("cook.com".to_string()),
    };
    Patient {
        id: "patient2".to_string(),
        national_id: Some("id88345".to_string()),
        addresses: vec![address.clone()],
        contact_details: vec![contact_details.clone()],
        date_of_birth: Some("1990-11-10".to_string()),
        date_of_birth_is_estimated: None,
        family: Some(Family {
            marital_status: Some(MaritalStatus::Single),
            caregiver: Some(person_2()),
            mother: Some(person_3()),
            next_of_kin: Some(person_1()),
        }),
        first_name: Some("Andy".to_string()),
        last_name: Some("Cook".to_string()),
        gender: Some(Gender::Male),
        health_center: None,
        passport_number: None,
        socio_economics: SocioEconomics {
            education: None,
            literate: None,
            occupation: None,
        },
        birthorder: None,
        hand: None,
    }
}

pub fn init_program_data(
    service_provider: &Arc<ServiceProvider>,
    site_id: u32,
) -> Result<(), RepositoryError> {
    let ctx = service_provider.context().unwrap();
    let connection = &ctx.connection;

    // patient
    let patient_schema_id = uuid();
    FormSchemaRowRepository::new(connection).upsert_one(&FormSchema {
        id: patient_schema_id.clone(),
        r#type: "JsonForms".to_string(),
        json_schema: serde_json::from_str(PATIENT_SCHEMA).unwrap(),
        ui_schema: serde_json::from_str(PATIENT_UI_SCHEMA).unwrap(),
    })?;
    DocumentRegistryRowRepository::new(connection).upsert_one(&DocumentRegistryRow {
        id: uuid(),
        document_type: PATIENT_TYPE.to_string(),
        context: DocumentContext::Patient,
        name: Some("Patient".to_string()),
        parent_id: None,
        form_schema_id: Some(patient_schema_id.clone()),
    })?;

    // program
    let program_schema_id = uuid();
    let placeholder_program_id = uuid();
    FormSchemaRowRepository::new(connection).upsert_one(&FormSchema {
        id: program_schema_id.clone(),
        r#type: "JsonForms".to_string(),
        json_schema: serde_json::from_str(PROGRAM_SCHEMA).unwrap(),
        ui_schema: serde_json::from_str(PROGRAM_UI_SCHEMA).unwrap(),
    })?;
    DocumentRegistryRowRepository::new(connection).upsert_one(&DocumentRegistryRow {
        id: placeholder_program_id.clone(),
        document_type: "PlaceholderProgram1".to_string(),
        context: DocumentContext::Program,
        name: Some("Placeholder program 1".to_string()),
        parent_id: None,
        form_schema_id: Some(program_schema_id),
    })?;

    // encounter
    let encounter_schema_id = uuid();
    FormSchemaRowRepository::new(connection).upsert_one(&FormSchema {
        id: encounter_schema_id.clone(),
        r#type: "JsonForms".to_string(),
        json_schema: serde_json::from_str(ENCOUNTER_SCHEMA).unwrap(),
        ui_schema: serde_json::from_str(ENCOUNTER_UI_SCHEMA).unwrap(),
    })?;
    DocumentRegistryRowRepository::new(connection).upsert_one(&DocumentRegistryRow {
        id: uuid(),
        document_type: "PlaceholderEncounter1".to_string(),
        context: DocumentContext::Encounter,
        name: Some("Placeholder encounter 1".to_string()),
        parent_id: Some(placeholder_program_id),
        form_schema_id: Some(encounter_schema_id),
    })?;

    // patients
    let store_id = StoreRepository::new(connection)
        .query_one(StoreFilter::new().site_id(EqualFilter::equal_to_i32(site_id as i32)))
        .unwrap()
        .unwrap()
        .store_row
        .id;
    let service = PatientService {};
    service
        .update_patient(
            &ctx,
            &service_provider,
            store_id.clone(),
            "no user",
            UpdatePatient {
                data: serde_json::to_value(patient_1()).unwrap(),
                schema_id: patient_schema_id.clone(),
                parent: None,
            },
        )
        .unwrap();
    let service = PatientService {};
    service
        .update_patient(
            &ctx,
            &service_provider,
            store_id.clone(),
            "no user",
            UpdatePatient {
                data: serde_json::to_value(patient_2()).unwrap(),
                schema_id: patient_schema_id,
                parent: None,
            },
        )
        .unwrap();

    Ok(())
}

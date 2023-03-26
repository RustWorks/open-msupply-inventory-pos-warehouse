use crate::sync::{
    test::integration::{
        central_server_configurations::NewSiteProperties, SyncRecordTester, TestStepData,
    },
    translations::{IntegrationRecords, PullDeleteRecord, PullDeleteRecordTable, PullUpsertRecord},
};

use repository::{Permission, UserPermissionRow};
use serde_json::json;
use util::uuid::uuid;

pub(crate) struct UserPermissionTester;

impl SyncRecordTester for UserPermissionTester {
    fn test_step_data(&self, new_site_properties: &NewSiteProperties) -> Vec<TestStepData> {
        let mut result = Vec::new();
        // STEP 1 - insert
        let store_id = new_site_properties.store_id.clone();

        let user_permission_row_1 = UserPermissionRow {
            id: uuid(),
            user_id: "test_user".to_string(),
            store_id: Some(store_id.clone()),
            permission: Permission::DocumentMutate,
            context: Some("some program".to_string()),
        };
        let user_permission_row_1_json = json!({
            "ID": user_permission_row_1.id,
            "user_ID": user_permission_row_1.user_id,
            "store_ID": user_permission_row_1.store_id,
            "permission": "DocumentMutate",
            "context_ID": user_permission_row_1.context
        });

        let user_permission_row_2 = UserPermissionRow {
            id: uuid(),
            user_id: "test_user".to_string(),
            store_id: Some(store_id),
            permission: Permission::DocumentQuery,
            context: Some("some program".to_string()),
        };
        let user_permission_row_2_json = json!({
            "ID": user_permission_row_2.id,
            "user_ID": user_permission_row_2.user_id,
            "store_ID": user_permission_row_2.store_id,
            "permission": "DocumentQuery",
            "context_ID": user_permission_row_2.context
        });

        result.push(TestStepData {
            central_upsert: json!({
                "om_user_permission": [user_permission_row_1_json, user_permission_row_2_json],
            }),
            central_delete: json!({}),
            integration_records: IntegrationRecords::from_upserts(vec![
                PullUpsertRecord::UserPermission(user_permission_row_1.clone()),
                PullUpsertRecord::UserPermission(user_permission_row_2),
            ]),
        });

        // STEP 2 - deletes
        result.push(TestStepData {
            central_upsert: json!({}),
            central_delete: json!({
                "om_user_permission": [user_permission_row_1.id],
            }),
            integration_records: IntegrationRecords::from_deletes(vec![PullDeleteRecord {
                id: user_permission_row_1.id,
                table: PullDeleteRecordTable::UserPermission,
            }]),
        });
        result
    }
}

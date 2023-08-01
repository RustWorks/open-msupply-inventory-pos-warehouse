use repository::{mock::context_program_a, Permission, UserPermissionRow};

use crate::sync::{
    test::TestSyncPullRecord,
    translations::{LegacyTableName, PullDeleteRecordTable, PullUpsertRecord},
};

const USER_PERMISSION_1: (&'static str, &'static str) = (
    "user_permission_1",
    r#"{
    "ID": "user_permission_1",
    "user_ID": "user_account_a",
    "store_ID": "store_a",
    "permission": "DocumentQuery",
    "context_ID": "program_a"
}"#,
);

const USER_PERMISSION_2: (&'static str, &'static str) = (
    "user_permission_2",
    r#"{
    "ID": "user_permission_2",
    "user_ID": "user_account_a",
    "store_ID": "store_a",
    "permission": "DocumentMutate",
    "context_ID": "program_a"
}"#,
);

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncPullRecord> {
    vec![
        TestSyncPullRecord::new_pull_upsert(
            LegacyTableName::USER_PERMISSION,
            USER_PERMISSION_1,
            PullUpsertRecord::UserPermission(UserPermissionRow {
                id: USER_PERMISSION_1.0.to_owned(),
                user_id: "user_account_a".to_string(),
                store_id: Some("store_a".to_string()),
                permission: Permission::DocumentQuery,
                context_id: Some(context_program_a().id.to_string()),
            }),
        ),
        TestSyncPullRecord::new_pull_upsert(
            LegacyTableName::USER_PERMISSION,
            USER_PERMISSION_2,
            PullUpsertRecord::UserPermission(UserPermissionRow {
                id: USER_PERMISSION_2.0.to_owned(),
                user_id: "user_account_a".to_string(),
                store_id: Some("store_a".to_string()),
                permission: Permission::DocumentMutate,
                context_id: Some(context_program_a().id.to_string()),
            }),
        ),
    ]
}

pub(crate) fn test_pull_delete_records() -> Vec<TestSyncPullRecord> {
    vec![TestSyncPullRecord::new_pull_delete(
        LegacyTableName::USER_PERMISSION,
        USER_PERMISSION_2.0,
        PullDeleteRecordTable::UserPermission,
    )]
}

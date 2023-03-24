use crate::sync::{
    test::TestSyncPullRecord,
    translations::{LegacyTableName, PullUpsertRecord},
};
use repository::{StorePreferenceRow, StorePreferenceType};

const STORE_PREFERENCE_1: (&'static str, &'static str) = (
    "store_preference",
    r#"{
    "ID": "store_preference",
    "store_ID": "store_a",
    "item": "store_preferences",
    "network_ID": "",
    "user_ID": "",
    "data": {
        "sort_batches_by_VVM_not_expiry": false,
        "new_patients_visible_in_this_store_only": true,
        "new_names_visible_in_this_store_only": true,
        "can_enter_total_distribution_quantities": false,
        "round_up_distribute_quantities": false,
        "can_pack_items_into_multiple_boxes": false,
        "can_issue_in_foreign_currency": false,
        "edit_sell_price_on_customer_invoice_lines": false,
        "purchase_order_must_be_authorised": false,
        "finalise_customer_invoices_automatically": false,
        "customer_invoices_must_be_authorised": false,
        "customer_invoice_authorisation_needed_only_if_over_budget": false,
        "confirm_customer_invoices_automatically": false,
        "supplier_invoices_must_be_authorised": false,
        "confirm_supplier_invoices_automatically": false,
        "goods_received_lines_must_be_authorised": false,
        "must_enter_locations_on_goods_received": false,
        "can_specify_manufacturer": false,
        "show_item_unit_column_while_issuing": false,
        "log_editing_transacts": false,
        "default_item_packsize_to_one": true,
        "shouldAuthoriseResponseRequisition": false,
        "includeRequisitionsInSuppliersRemoteAuthorisationProcesses": false,
        "canLinkRequistionToSupplierInvoice": false,
        "responseRequisitionAutoFillSupplyQuantity": false,
        "useExtraFieldsForRequisitions": false,
        "CommentFieldToBeShownOnSupplierInvoiceLines": false,
        "UseEDDPlaceholderLinesOnSupplierInvoice": false,
        "consolidateBatches": false,
        "editPrescribedQuantityOnPrescription": false,
        "chooseDiagnosisOnPrescription": false,
        "useConsumptionAndStockFromCustomersForInternalOrders": false,
        "alertIfDispensingSameVaccine": false,
        "monthlyConsumptionEnforceLookBackPeriod": false,
        "usesVaccineModule": false,
        "usesDashboardModule": false,
        "usesCashRegisterModule": false,
        "usesPaymentModule": false,
        "usesPatientTypes": false,
        "usesHideSnapshotColumn": false,
        "pickfaceReplenishmentsMustAuthorised": false,
        "ableToSpecifyVVMStatusWhenReceivingItems": false,
        "good_receipt_finalise_next_action": "supplier_invoice_on_hold",
        "stock_transfer_supplier_invoice_is_on_hold": true,
        "monthlyConsumptionLookBackPeriod": "0",
        "monthsLeadTime": "0",
        "usesDispensaryModule": false,
        "monthsOverstock": 12,
        "monthsUnderstock": 4,
        "monthsItemsExpire": 2,
        "boxPrefix": "",
        "boxPercentageSpace": 0
    }
}"#,
);

const STORE_PREFERENCE_2: (&'static str, &'static str) = (
    "store_preference_2",
    r#"{
    "ID": "store_preference_2",
    "store_ID": "store_b",
    "item": "store_preferences",
    "network_ID": "",
    "user_ID": "",
    "data": {
        "sort_batches_by_VVM_not_expiry": false,
        "new_patients_visible_in_this_store_only": true,
        "new_names_visible_in_this_store_only": true,
        "can_enter_total_distribution_quantities": false,
        "round_up_distribute_quantities": false,
        "can_pack_items_into_multiple_boxes": false,
        "can_issue_in_foreign_currency": false,
        "edit_sell_price_on_customer_invoice_lines": false,
        "purchase_order_must_be_authorised": false,
        "finalise_customer_invoices_automatically": false,
        "customer_invoices_must_be_authorised": false,
        "customer_invoice_authorisation_needed_only_if_over_budget": false,
        "confirm_customer_invoices_automatically": false,
        "supplier_invoices_must_be_authorised": false,
        "confirm_supplier_invoices_automatically": false,
        "goods_received_lines_must_be_authorised": false,
        "must_enter_locations_on_goods_received": false,
        "can_specify_manufacturer": false,
        "show_item_unit_column_while_issuing": false,
        "log_editing_transacts": false,
        "default_item_packsize_to_one": false,
        "shouldAuthoriseResponseRequisition": false,
        "includeRequisitionsInSuppliersRemoteAuthorisationProcesses": false,
        "canLinkRequistionToSupplierInvoice": false,
        "responseRequisitionAutoFillSupplyQuantity": false,
        "useExtraFieldsForRequisitions": false,
        "CommentFieldToBeShownOnSupplierInvoiceLines": false,
        "UseEDDPlaceholderLinesOnSupplierInvoice": false,
        "consolidateBatches": false,
        "editPrescribedQuantityOnPrescription": false,
        "chooseDiagnosisOnPrescription": false,
        "useConsumptionAndStockFromCustomersForInternalOrders": false,
        "alertIfDispensingSameVaccine": false,
        "monthlyConsumptionEnforceLookBackPeriod": false,
        "usesVaccineModule": false,
        "usesDashboardModule": false,
        "usesCashRegisterModule": false,
        "usesPaymentModule": false,
        "usesPatientTypes": false,
        "usesHideSnapshotColumn": false,
        "pickfaceReplenishmentsMustAuthorised": false,
        "ableToSpecifyVVMStatusWhenReceivingItems": false,
        "good_receipt_finalise_next_action": "supplier_invoice_on_hold",
        "stock_transfer_supplier_invoice_is_on_hold": true,
        "monthlyConsumptionLookBackPeriod": "0",
        "monthsLeadTime": "0",
        "usesDispensaryModule": false,
        "monthsOverstock": 6,
        "monthsUnderstock": 3,
        "monthsItemsExpire": 3,
        "boxPrefix": "",
        "boxPercentageSpace": 0
    }
}"#,
);

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncPullRecord> {
    vec![
        TestSyncPullRecord::new_pull_upsert(
            LegacyTableName::STORE_PREFERENCE,
            STORE_PREFERENCE_1,
            PullUpsertRecord::StorePreference(StorePreferenceRow {
                id: "store_a".to_string(),
                r#type: StorePreferenceType::StorePreferences,
                pack_to_one: true,
                requisitions_require_supplier_authorisation: true,
                use_authorisation_for_customer_requisitions: true,
            }),
        ),
        TestSyncPullRecord::new_pull_upsert(
            LegacyTableName::STORE_PREFERENCE,
            STORE_PREFERENCE_2,
            PullUpsertRecord::StorePreference(StorePreferenceRow {
                id: "store_b".to_string(),
                r#type: StorePreferenceType::StorePreferences,
                pack_to_one: false,
                requisitions_require_supplier_authorisation: false,
                use_authorisation_for_customer_requisitions: false,
            }),
        ),
    ]
}

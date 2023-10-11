use std::collections::HashMap;

#[derive(Clone)]
pub enum Permissions {
    CreatePurchaseOrders,
    ViewPurchaseOrders,
    EditPurchaseOrders,
    DeletePurchaseOrders,
    UserCannotViewPrices,
    FinaliseCustomerInvoices,
    ViewCostPriceOnBuild,
    ViewBillOfMaterials,
    EditBillOfMaterials,
    FinalizeBuilds,
    EditDeleteQuotes,
    LogOnInStoreMode,
    LogOnInDispensaryMode,
    ManageDrugInteractionGroups,
    ViewAndPrintLabels,
    EnterInventoryAdjustments,
    CanEditCommentsOnFinalizedInvoices,
    EditItemNamesCodesAndUnits,
    SeePricingOnItemInputForm,
    ViewDDDInformation,
    CreateNewQuotes,
    ManageReports,
    RevertReportsToOriginal,
    EditNameCodes,
    EditNameCategories,
    ViewGoodsReceived,
    EditPurchaseOrderPricing,
    EditItemUnitsList,
    ConfirmPurchaseOrders,
    PrintDuplicatesOnCustomerInvoice,
    EnterWebPasswordsForNames,
    EditAndCreateWebMessages,
    EditInventoryAdjustments,
    CreateCustomerSupplierManufacturerNames,
    ViewCustomerSupplierManufacturerNames,
    FinalisePurchaseOrders,
    CreateSupplierInvoices,
    ViewSupplierInvoices,
    CreateNewItems,
    CreateEditBackorders,
    CreateCustomerInvoices,
    ViewCustomerInvoices,
    AddEditGoodsReceived,
    ManageTenders,
    AddPatients,
    ViewPatients,
    /// not used
    EditRemoteData,
    ChooseDispensaryModeByDefaultOnLogIn,
    CreateRepacksOrSplitStock,
    AddEditDepartments,
    EditRepacks,
    BuildItems,
    EditBuildItems,
    MakeCashPayments,
    ReceiveCash,
    ImportSupplierInvoices,
    CreateEditPatientEvents,
    CreateEditTenders,
    TransferGoodsBetweenStores,
    DuplicateSupplierCustomerInvoices,
    DuplicatePurchaseOrder,
    EditPatientDetails,
    ViewReports,
    AddEditCurrencies,
    AddEditReminders,
    AddEditMiscLabels,
    AddEditAbbreviations,
    AddEditWarnings,
    AddEditPrescribers,
    AddEditTransactionCategories,
    AddEditContacts,
    MergePrescribers,
    /// bandana 080115 user permission set for  Admin tab
    AddEditUsers,
    SendEmail,
    ViewLog,
    Spare,
    ViewEditPreferences,
    FinaliseMultipleInvoices,
    MergeTwoItems,
    ExportImport,
    AddEditNameGroups,
    ManageLocations,
    ManageItemAccess,
    ModifySellAndCostPricesOfExistingStock,
    ViewItems,
    FinaliseSupplierInvoices,
    FinaliseStockTransfers,
    FinaliseRepacks,
    FinaliseGoodsReceived,
    FinaliseInventoryAdjustments,
    EditNameChargeCode,
    CancelFinalisedInvoices,
    MakeItemInActive,
    EditUserFieldsOnFinalisedInvoices,
    Spare2,
    UpdatePackSizeCostAndSellPrice,
    ViewAssets,
    UploadTenderDocument,
    DownloadTenderDocument,
    DeleteTenderDocument,
    UploadQuoteDocument,
    DownloadQuoteDocument,
    DeleteQuoteDocument,
    UploadBatchDocument,
    DownloadBatchDocument,
    DeleteBatchDocument,
    AddStocktakeLines,
    ViewStocktakeLines,
    EditStocktakeLines,
    DeleteStocktakeLines,
    CreateStocktake,
    DeleteStocktake,
    UpdateMasterCode,
    MergeNames,
    EditAndDeleteRemindersAssignedToMe,
    CreateNewStores,
    EditStoreDetails,
    ViewInventoryAdjustments,
    EditCustomerInvoices,
    EditSupplierInvoices,
    EditItems,
    EditCustomerSupplierManufacturerNames,
    ChangeInvoiceCategoryOnFinalisedInvoice,
    CustomerStockTakesShowInternalAnalysisColumnsByDefault,
    DuplicateItems,
    EditStocktakeDates,
    BackupDataFile,
    EditVisibilityInStores,
    DeleteItems,
    DeleteNames,
    ChangeTransportationDatesOnFinalisedInvoice,
    AuthorisePurchaseOrders,
    AddEditAssets,
    SetupAssets,
    AuthoriseCustomerInvoices,
    AuthoriseSupplierInvoices,
    AuthoriseGoodsReceived,
    /// via the website
    ModifyQuotesEnteredByTheSupplier,
    PrintPurchaseOrders,
    EditItemDefaultPrice,
    AddImportCustomerBudgets,
    EditDeleteCustomerBudgets,
    ConsolidateStock,
    EditStock,
    EditPaymentNoteField,
    CreateAndEditCustomStockFieldValues,
    AddEditRegistrations,
    ChangeRegistrationStatus,
    AddEditMasterList,
    ViewRequisitions,
    CreateAndEditRequisitions,
    AdministerFinalisedReportTypeRequisitions,
    ViewStock,
    AccessServerAdministration,
    CanEditAuthorisers,
    CloneDatabase,
    CanEditInsuranceProviders,
    CreateCashTransactions,
    CanEditPeriodsPeriodSchedules,
    CanAddAndEditOptions,
    CanAddAndEditInsurancePolicies,
    CanViewAndEditSupplierHubDetails,
    ReturnStockFromSupplierInvoices,
    CreateSupplierCredits,
    CreateCustomerInvoicesFromRequisitions,
    AddEditSyncSites,
    ViewAndEditVaccineVialMonitorStatus,
    ViewAndEditTemperatureBreachConfiguration,
    OverwriteTotalAmountInPrescriptions,
    ViewSensorDetails,
    EditSensorLocation,
    EditStoreCredentials,
    ChangeAssetStatus,
    AddEditVaccinators,
    HISAddPatients,
    HISEditPatientsInfo,
    HISCreateEncounters,
    HISMovePatients,
    HISDischargePatients,
    HISCreateWards,
    HISCreateBeds,
    HISEditWardDetail,
    HISEditBedDetail,
    HISCreateICDCodes,
    HISEditICDCodes,
    HISMergePatients,
    HISAddEncounterDiseases,
    HISEditEncounterDiseases,
    HISAddProcedure,
    ConfirmInternalOrderSent,
}

pub fn permission_mapping() -> HashMap<i16, Permissions> {
    HashMap::from([
        (1, Permissions::CreatePurchaseOrders),
        (2, Permissions::ViewPurchaseOrders),
        (3, Permissions::EditPurchaseOrders),
        (4, Permissions::DeletePurchaseOrders),
        (5, Permissions::UserCannotViewPrices),
        (6, Permissions::FinaliseCustomerInvoices),
        (7, Permissions::ViewCostPriceOnBuild),
        (8, Permissions::ViewBillOfMaterials),
        (9, Permissions::EditBillOfMaterials),
        (10, Permissions::FinalizeBuilds),
        (11, Permissions::EditDeleteQuotes),
        (12, Permissions::LogOnInStoreMode),
        (13, Permissions::LogOnInDispensaryMode),
        (14, Permissions::ManageDrugInteractionGroups),
        (15, Permissions::ViewAndPrintLabels),
        (16, Permissions::EnterInventoryAdjustments),
        (17, Permissions::CanEditCommentsOnFinalizedInvoices),
        (18, Permissions::EditItemNamesCodesAndUnits),
        (19, Permissions::SeePricingOnItemInputForm),
        (20, Permissions::ViewDDDInformation),
        (21, Permissions::CreateNewQuotes),
        (22, Permissions::ManageReports),
        (23, Permissions::RevertReportsToOriginal),
        (24, Permissions::EditNameCodes),
        (25, Permissions::EditNameCategories),
        (26, Permissions::ViewGoodsReceived),
        (27, Permissions::EditPurchaseOrderPricing),
        (28, Permissions::EditItemUnitsList),
        (29, Permissions::ConfirmPurchaseOrders),
        (31, Permissions::PrintDuplicatesOnCustomerInvoice),
        (32, Permissions::EnterWebPasswordsForNames),
        (33, Permissions::EditAndCreateWebMessages),
        (34, Permissions::EditInventoryAdjustments),
        (35, Permissions::CreateCustomerSupplierManufacturerNames),
        (36, Permissions::ViewCustomerSupplierManufacturerNames),
        (37, Permissions::FinalisePurchaseOrders),
        (38, Permissions::CreateSupplierInvoices),
        (39, Permissions::ViewSupplierInvoices),
        (40, Permissions::CreateNewItems),
        (41, Permissions::CreateEditBackorders),
        (42, Permissions::CreateCustomerInvoices),
        (43, Permissions::ViewCustomerInvoices),
        (44, Permissions::AddEditGoodsReceived),
        (45, Permissions::ManageTenders),
        (46, Permissions::AddPatients),
        (47, Permissions::EditRemoteData),
        (48, Permissions::ChooseDispensaryModeByDefaultOnLogIn),
        (49, Permissions::CreateRepacksOrSplitStock),
        (50, Permissions::AddEditDepartments),
        (51, Permissions::EditRepacks),
        (52, Permissions::BuildItems),
        (53, Permissions::EditBuildItems),
        (54, Permissions::MakeCashPayments),
        (55, Permissions::ReceiveCash),
        (56, Permissions::ImportSupplierInvoices),
        (57, Permissions::CreateEditPatientEvents),
        (58, Permissions::CreateEditTenders),
        (59, Permissions::TransferGoodsBetweenStores),
        (60, Permissions::DuplicateSupplierCustomerInvoices),
        (61, Permissions::DuplicatePurchaseOrder),
        (62, Permissions::EditPatientDetails),
        (63, Permissions::ViewReports),
        (64, Permissions::AddEditCurrencies),
        (65, Permissions::AddEditReminders),
        (66, Permissions::AddEditMiscLabels),
        (67, Permissions::AddEditAbbreviations),
        (68, Permissions::AddEditWarnings),
        (69, Permissions::AddEditPrescribers),
        (70, Permissions::AddEditTransactionCategories),
        (71, Permissions::AddEditContacts),
        (72, Permissions::MergePrescribers),
        (73, Permissions::AddEditUsers),
        (74, Permissions::SendEmail),
        (76, Permissions::ViewLog),
        (78, Permissions::Spare),
        (79, Permissions::ViewEditPreferences),
        (80, Permissions::FinaliseMultipleInvoices),
        (81, Permissions::MergeTwoItems),
        (82, Permissions::ExportImport),
        (83, Permissions::AddEditNameGroups),
        (84, Permissions::ManageLocations),
        (85, Permissions::ManageItemAccess),
        (86, Permissions::ModifySellAndCostPricesOfExistingStock),
        (87, Permissions::ViewItems),
        (88, Permissions::FinaliseSupplierInvoices),
        (89, Permissions::FinaliseStockTransfers),
        (90, Permissions::FinaliseRepacks),
        (91, Permissions::FinaliseGoodsReceived),
        (92, Permissions::FinaliseInventoryAdjustments),
        (93, Permissions::EditNameChargeCode),
        (94, Permissions::CancelFinalisedInvoices),
        (95, Permissions::MakeItemInActive),
        (96, Permissions::EditUserFieldsOnFinalisedInvoices),
        (97, Permissions::Spare2),
        (98, Permissions::UpdatePackSizeCostAndSellPrice),
        (99, Permissions::ViewAssets),
        (100, Permissions::UploadTenderDocument),
        (101, Permissions::DownloadTenderDocument),
        (102, Permissions::DeleteTenderDocument),
        (103, Permissions::UploadQuoteDocument),
        (104, Permissions::DownloadQuoteDocument),
        (105, Permissions::DeleteQuoteDocument),
        (106, Permissions::UploadBatchDocument),
        (107, Permissions::DownloadBatchDocument),
        (108, Permissions::DeleteBatchDocument),
        (109, Permissions::AddStocktakeLines),
        // ViewStocktakeLines has been removed from mSupply
        // (110, Permissions::ViewStocktakeLines),
        (111, Permissions::EditStocktakeLines),
        (112, Permissions::DeleteStocktakeLines),
        (113, Permissions::CreateStocktake),
        (114, Permissions::DeleteStocktake),
        (115, Permissions::UpdateMasterCode),
        (116, Permissions::MergeNames),
        (117, Permissions::EditAndDeleteRemindersAssignedToMe),
        (118, Permissions::CreateNewStores),
        (119, Permissions::EditStoreDetails),
        (120, Permissions::ViewInventoryAdjustments),
        (121, Permissions::EditCustomerInvoices),
        (122, Permissions::EditSupplierInvoices),
        (123, Permissions::EditItems),
        (124, Permissions::EditCustomerSupplierManufacturerNames),
        (125, Permissions::ChangeInvoiceCategoryOnFinalisedInvoice),
        (
            126,
            Permissions::CustomerStockTakesShowInternalAnalysisColumnsByDefault,
        ),
        (127, Permissions::DuplicateItems),
        (128, Permissions::EditStocktakeDates),
        (129, Permissions::BackupDataFile),
        (130, Permissions::EditVisibilityInStores),
        (131, Permissions::DeleteItems),
        (132, Permissions::DeleteNames),
        (
            133,
            Permissions::ChangeTransportationDatesOnFinalisedInvoice,
        ),
        (134, Permissions::AuthorisePurchaseOrders),
        (135, Permissions::AddEditAssets),
        (136, Permissions::SetupAssets),
        (137, Permissions::AuthoriseCustomerInvoices),
        (138, Permissions::AuthoriseSupplierInvoices),
        (139, Permissions::AuthoriseGoodsReceived),
        (140, Permissions::ModifyQuotesEnteredByTheSupplier),
        (141, Permissions::PrintPurchaseOrders),
        (142, Permissions::EditItemDefaultPrice),
        (143, Permissions::AddImportCustomerBudgets),
        (144, Permissions::EditDeleteCustomerBudgets),
        (146, Permissions::ConsolidateStock),
        (147, Permissions::EditStock),
        (148, Permissions::EditPaymentNoteField),
        (149, Permissions::CreateAndEditCustomStockFieldValues),
        (150, Permissions::AddEditRegistrations),
        (151, Permissions::ChangeRegistrationStatus),
        (152, Permissions::AddEditMasterList),
        (153, Permissions::ViewRequisitions),
        (154, Permissions::CreateAndEditRequisitions),
        (155, Permissions::AdministerFinalisedReportTypeRequisitions),
        (156, Permissions::ViewStock),
        (157, Permissions::AccessServerAdministration),
        (158, Permissions::CanEditAuthorisers),
        (160, Permissions::CloneDatabase),
        (161, Permissions::CanEditInsuranceProviders),
        (162, Permissions::CreateCashTransactions),
        (163, Permissions::CanEditPeriodsPeriodSchedules),
        (164, Permissions::CanAddAndEditOptions),
        (166, Permissions::CanAddAndEditInsurancePolicies),
        (167, Permissions::CanViewAndEditSupplierHubDetails),
        (168, Permissions::ReturnStockFromSupplierInvoices),
        (169, Permissions::CreateSupplierCredits),
        (170, Permissions::CreateCustomerInvoicesFromRequisitions),
        (171, Permissions::AddEditSyncSites),
        (172, Permissions::ViewAndEditVaccineVialMonitorStatus),
        (173, Permissions::ViewAndEditTemperatureBreachConfiguration),
        (174, Permissions::OverwriteTotalAmountInPrescriptions),
        (175, Permissions::ViewSensorDetails),
        (176, Permissions::EditSensorLocation),
        (177, Permissions::EditStoreCredentials),
        (178, Permissions::ChangeAssetStatus),
        (179, Permissions::AddEditVaccinators),
        (190, Permissions::ViewPatients),
        (200, Permissions::ConfirmInternalOrderSent),
        (201, Permissions::ColdChainApi),
        (501, Permissions::HISAddPatients),
        (502, Permissions::HISEditPatientsInfo),
        (503, Permissions::HISCreateEncounters),
        (504, Permissions::HISMovePatients),
        (505, Permissions::HISDischargePatients),
        (506, Permissions::HISCreateWards),
        (507, Permissions::HISCreateBeds),
        (508, Permissions::HISEditWardDetail),
        (509, Permissions::HISEditBedDetail),
        (510, Permissions::HISCreateICDCodes),
        (511, Permissions::HISEditICDCodes),
        (512, Permissions::HISMergePatients),
        (513, Permissions::HISAddEncounterDiseases),
        (514, Permissions::HISEditEncounterDiseases),
        (515, Permissions::HISAddProcedure),
    ])
}

pub fn map_api_permissions(permissions: Vec<bool>) -> Vec<Permissions> {
    let mut output = Vec::new();
    let mapping = permission_mapping();
    for (i, has_per) in permissions.iter().enumerate() {
        if !has_per {
            continue;
        }
        if let Some(permission) = mapping.get(&((i + 1) as i16)) {
            output.push(permission.clone())
        }
    }
    output
}

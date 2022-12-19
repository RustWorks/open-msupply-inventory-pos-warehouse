use async_graphql::*;
use chrono::DateTime;
use chrono::Utc;
use graphql_core::pagination::PaginationInput;
use mutations::allocate_number::allocate_program_number;
use mutations::allocate_number::AllocateProgramNumberInput;
use mutations::allocate_number::AllocateProgramNumberResponse;
use mutations::delete_document::delete_document;
use mutations::delete_document::DeleteDocumentInput;
use mutations::delete_document::DeleteDocumentResponse;
use mutations::encounter::insert::insert_encounter;
use mutations::encounter::insert::InsertEncounterInput;
use mutations::encounter::insert::InsertEncounterResponse;
use mutations::encounter::update::update_encounter;
use mutations::encounter::update::UpdateEncounterInput;
use mutations::encounter::update::UpdateEncounterResponse;
use mutations::insert_document_registry::*;
use mutations::insert_form_schema::*;
use mutations::patient::insert::*;
use mutations::patient::update::update_patient;
use mutations::patient::update::UpdatePatientInput;
use mutations::patient::update::UpdatePatientResponse;
use mutations::program_enrolment::insert::insert_program_enrolment;
use mutations::program_enrolment::insert::InsertProgramEnrolmentInput;
use mutations::program_enrolment::insert::InsertProgramEnrolmentResponse;
use mutations::program_enrolment::update::update_program_enrolment;
use mutations::program_enrolment::update::UpdateProgramEnrolmentInput;
use mutations::program_enrolment::update::UpdateProgramEnrolmentResponse;
use mutations::undelete_document::undelete_document;
use mutations::undelete_document::UndeleteDocumentInput;
use mutations::undelete_document::UndeleteDocumentResponse;
use mutations::update_document::*;
use types::document::DocumentNode;
use types::json_schema::FormSchemaNode;
use types::program_enrolment::ProgramEventFilterInput;

mod mutations;

mod queries;
use self::queries::*;

mod types;

#[derive(Default, Clone)]
pub struct ProgramsQueries;

#[Object]
impl ProgramsQueries {
    pub async fn documents(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Store id")] store_id: String,
        page: Option<PaginationInput>,
        #[graphql(desc = "The document filter")] filter: Option<DocumentFilterInput>,
        sort: Option<DocumentSortInput>,
    ) -> Result<DocumentResponse> {
        documents(ctx, store_id, page, filter, sort)
    }

    pub async fn document(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Store id")] store_id: String,
        #[graphql(desc = "The document name")] name: String,
    ) -> Result<Option<DocumentNode>> {
        document(ctx, store_id, name)
    }

    pub async fn document_history(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Store id")] store_id: String,
        #[graphql(desc = "The document name")] name: String,
    ) -> Result<DocumentHistoryResponse> {
        document_history(ctx, store_id, name)
    }

    pub async fn document_registries(
        &self,
        ctx: &Context<'_>,
        filter: Option<DocumentRegistryFilterInput>,
        sort: Option<Vec<DocumentRegistrySortInput>>,
    ) -> Result<DocumentRegistryResponse> {
        document_registries(ctx, filter, sort)
    }

    pub async fn form_schema(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<FormSchemaNode>> {
        form_schema(ctx, id)
    }

    pub async fn patients(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        page: Option<PaginationInput>,
        filter: Option<PatientFilterInput>,
        sort: Option<Vec<PatientSortInput>>,
    ) -> Result<PatientResponse> {
        patients(ctx, store_id, page, filter, sort)
    }
    pub async fn patient(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        patient_id: String,
    ) -> Result<Option<PatientNode>> {
        patient(ctx, store_id, patient_id)
    }
    pub async fn patient_search(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: PatientSearchInput,
    ) -> Result<PatientSearchResponse> {
        patient_search(ctx, store_id, input)
    }

    pub async fn program_enrolments(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        sort: Option<ProgramEnrolmentSortInput>,
        filter: Option<ProgramEnrolmentFilterInput>,
    ) -> Result<ProgramEnrolmentResponse> {
        program_enrolments(ctx, store_id, sort, filter)
    }

    pub async fn program_events(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        patient_id: String,
        at: Option<DateTime<Utc>>,
        page: Option<PaginationInput>,
        sort: Option<ProgramEventSortInput>,
        filter: Option<ProgramEventFilterInput>,
    ) -> Result<ProgramEventResponse> {
        program_events(ctx, store_id, patient_id, at, page, sort, filter)
    }

    pub async fn encounters(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        page: Option<PaginationInput>,
        filter: Option<EncounterFilterInput>,
        sort: Option<EncounterSortInput>,
    ) -> Result<EncounterResponse> {
        encounters(ctx, store_id, page, filter, sort)
    }

    pub async fn encounter_fields(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: EncounterFieldsInput,
        page: Option<PaginationInput>,
        filter: Option<EncounterFilterInput>,
        sort: Option<EncounterSortInput>,
    ) -> Result<EncounterFieldsResponse> {
        encounter_fields(ctx, store_id, input, page, filter, sort)
    }
}

#[derive(Default, Clone)]
pub struct ProgramsMutations;

#[Object]
impl ProgramsMutations {
    async fn update_document(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: UpdateDocumentInput,
    ) -> Result<UpdateDocumentResponse> {
        update_document(ctx, store_id, input)
    }

    async fn delete_document(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: DeleteDocumentInput,
    ) -> Result<DeleteDocumentResponse> {
        delete_document(ctx, store_id, input)
    }

    async fn undelete_document(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: UndeleteDocumentInput,
    ) -> Result<UndeleteDocumentResponse> {
        undelete_document(ctx, store_id, input)
    }

    async fn insert_document_registry(
        &self,
        ctx: &Context<'_>,
        input: InsertDocumentRegistryInput,
    ) -> Result<InsertDocumentResponse> {
        insert_document_registry(ctx, input)
    }

    async fn insert_form_schema(
        &self,
        ctx: &Context<'_>,

        input: InsertFormSchemaInput,
    ) -> Result<InsertFormSchemaResponse> {
        insert_form_schema(ctx, input)
    }

    pub async fn insert_patient(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: InsertPatientInput,
    ) -> Result<InsertPatientResponse> {
        insert_patient(ctx, store_id, input)
    }

    pub async fn update_patient(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: UpdatePatientInput,
    ) -> Result<UpdatePatientResponse> {
        update_patient(ctx, store_id, input)
    }

    /// Enrols a patient into a program by adding a program document to the patient's documents.
    /// Every patient can only have one program document of each program type.
    pub async fn insert_program_enrolment(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: InsertProgramEnrolmentInput,
    ) -> Result<InsertProgramEnrolmentResponse> {
        insert_program_enrolment(ctx, store_id, input)
    }

    /// Updates an existing program document belonging to a patient.
    pub async fn update_program_enrolment(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: UpdateProgramEnrolmentInput,
    ) -> Result<UpdateProgramEnrolmentResponse> {
        update_program_enrolment(ctx, store_id, input)
    }

    pub async fn insert_encounter(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: InsertEncounterInput,
    ) -> Result<InsertEncounterResponse> {
        insert_encounter(ctx, store_id, input)
    }

    pub async fn update_encounter(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: UpdateEncounterInput,
    ) -> Result<UpdateEncounterResponse> {
        update_encounter(ctx, store_id, input)
    }

    pub async fn allocate_program_number(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: AllocateProgramNumberInput,
    ) -> Result<AllocateProgramNumberResponse> {
        allocate_program_number(ctx, store_id, input)
    }
}
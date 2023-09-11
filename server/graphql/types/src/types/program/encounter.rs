use async_graphql::{dataloader::DataLoader, *};
use chrono::{DateTime, Utc};
use graphql_core::{
    generic_filters::{DatetimeFilterInput, EqualFilterStringInput, StringFilterInput},
    loader::{
        ClinicianLoader, ClinicianLoaderInput, DocumentLoader, PatientLoader,
        ProgramEnrolmentLoader, ProgramEnrolmentLoaderInput,
    },
    map_filter,
    pagination::PaginationInput,
    standard_graphql_error::StandardGraphqlError,
    ContextExt,
};
use repository::{
    DatetimeFilter, Encounter, EncounterFilter, EncounterSort, EncounterSortField, EncounterStatus,
    EqualFilter, PaginationOption, ProgramEventFilter, ProgramEventSortField, Sort, StringFilter,
};
use serde::Serialize;

use crate::types::ClinicianNode;

use super::{
    document::DocumentNode,
    patient::PatientNode,
    program_enrolment::ProgramEnrolmentNode,
    program_event::{
        ProgramEventConnector, ProgramEventNode, ProgramEventResponse, ProgramEventSortInput,
    },
};

pub struct EncounterNode {
    pub store_id: String,
    pub encounter: Encounter,
    pub allowed_ctx: Vec<String>,
}

#[derive(SimpleObject)]
pub struct EncounterConnector {
    pub total_count: u32,
    pub nodes: Vec<EncounterNode>,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterEncounterStatusInput {
    pub equal_to: Option<EncounterNodeStatus>,
    pub equal_any: Option<Vec<EncounterNodeStatus>>,
    pub not_equal_to: Option<EncounterNodeStatus>,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum EncounterSortFieldInput {
    Type,
    PatientId,
    Program,
    CreatedDatetime,
    StartDatetime,
    EndDatetime,
    Status,
}

#[derive(InputObject)]
pub struct EncounterSortInput {
    /// Sort query result by `key`
    key: EncounterSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}

impl EncounterSortInput {
    pub fn to_domain(self) -> EncounterSort {
        let key = match self.key {
            EncounterSortFieldInput::Type => EncounterSortField::DocumentType,
            EncounterSortFieldInput::PatientId => EncounterSortField::PatientId,
            EncounterSortFieldInput::Program => EncounterSortField::Context,
            EncounterSortFieldInput::CreatedDatetime => EncounterSortField::CreatedDatetime,
            EncounterSortFieldInput::StartDatetime => EncounterSortField::StartDatetime,
            EncounterSortFieldInput::EndDatetime => EncounterSortField::EndDatetime,
            EncounterSortFieldInput::Status => EncounterSortField::Status,
        };

        EncounterSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct EncounterFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub r#type: Option<EqualFilterStringInput>,
    pub patient_id: Option<EqualFilterStringInput>,
    /// The program id
    pub program_id: Option<EqualFilterStringInput>,
    pub created_datetime: Option<DatetimeFilterInput>,
    pub start_datetime: Option<DatetimeFilterInput>,
    pub end_datetime: Option<DatetimeFilterInput>,
    pub status: Option<EqualFilterEncounterStatusInput>,
    pub clinician_id: Option<EqualFilterStringInput>,
    pub document_name: Option<EqualFilterStringInput>,
    pub document_data: Option<StringFilterInput>,
}

impl EncounterFilterInput {
    pub fn to_domain_filter(self) -> EncounterFilter {
        EncounterFilter {
            id: self.id.map(EqualFilter::from),
            patient_id: self.patient_id.map(EqualFilter::from),
            program_id: self.program_id.map(EqualFilter::from),
            created_datetime: self.created_datetime.map(DatetimeFilter::from),
            start_datetime: self.start_datetime.map(DatetimeFilter::from),
            status: self
                .status
                .map(|s| map_filter!(s, EncounterNodeStatus::to_domain)),
            end_datetime: self.end_datetime.map(DatetimeFilter::from),
            clinician_id: self.clinician_id.map(EqualFilter::from),
            document_type: self.r#type.map(EqualFilter::from),
            document_name: self.document_name.map(EqualFilter::from),
            document_data: self.document_data.map(StringFilter::from),
            program_context_id: None,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // only needed to be comparable in tests
pub enum EncounterNodeStatus {
    Pending,
    Visited,
    Cancelled,
}

impl EncounterNodeStatus {
    pub fn to_domain(self) -> EncounterStatus {
        match self {
            EncounterNodeStatus::Pending => EncounterStatus::Pending,
            EncounterNodeStatus::Visited => EncounterStatus::Visited,
            EncounterNodeStatus::Cancelled => EncounterStatus::Cancelled,
        }
    }

    pub fn from_domain(status: &EncounterStatus) -> EncounterNodeStatus {
        match status {
            EncounterStatus::Pending => EncounterNodeStatus::Pending,
            EncounterStatus::Visited => EncounterNodeStatus::Visited,
            EncounterStatus::Cancelled => EncounterNodeStatus::Cancelled,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct ActiveEncounterEventFilterInput {
    pub r#type: Option<EqualFilterStringInput>,
    pub data: Option<StringFilterInput>,
    /// Only include events that are for the current encounter, i.e. have matching encounter type
    /// and matching encounter name of the current encounter. If not set all events with matching
    /// encounter type are returned.
    pub is_current_encounter: Option<bool>,
}

impl ActiveEncounterEventFilterInput {
    pub fn to_domain(self) -> ProgramEventFilter {
        let ActiveEncounterEventFilterInput {
            r#type,
            data,
            is_current_encounter: _,
        } = self;
        ProgramEventFilter {
            datetime: None,
            active_start_datetime: None,
            active_end_datetime: None,
            patient_id: None,
            document_type: None,
            document_name: None,
            r#type: r#type.map(EqualFilter::from),
            data: data.map(StringFilter::from),
            context_id: None,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct EncounterEventFilterInput {
    pub r#type: Option<EqualFilterStringInput>,
    pub data: Option<StringFilterInput>,
    pub datetime: Option<DatetimeFilterInput>,
    pub active_start_datetime: Option<DatetimeFilterInput>,
    pub active_end_datetime: Option<DatetimeFilterInput>,

    /// Only include events that are for the current encounter, i.e. have matching encounter type
    /// and matching encounter name of the current encounter. If not set all events with matching
    /// encounter type are returned.
    pub is_current_encounter: Option<bool>,
}

impl EncounterEventFilterInput {
    pub fn to_domain(self) -> ProgramEventFilter {
        let EncounterEventFilterInput {
            r#type,
            data,
            datetime,
            active_start_datetime,
            active_end_datetime,
            is_current_encounter: _,
        } = self;
        ProgramEventFilter {
            datetime: datetime.map(DatetimeFilter::from),
            active_start_datetime: active_start_datetime.map(DatetimeFilter::from),
            active_end_datetime: active_end_datetime.map(DatetimeFilter::from),
            patient_id: None,
            document_type: None,
            document_name: None,
            r#type: r#type.map(EqualFilter::from),
            data: data.map(StringFilter::from),
            context_id: None,
        }
    }
}

#[Object]
impl EncounterNode {
    pub async fn id(&self) -> &str {
        &self.encounter.0.id
    }

    pub async fn context_id(&self) -> &str {
        &self.encounter.1.context_id
    }

    pub async fn program_id(&self) -> &str {
        &self.encounter.0.program_id
    }

    pub async fn patient_id(&self) -> &str {
        &self.encounter.0.patient_id
    }

    pub async fn patient(&self, ctx: &Context<'_>) -> Result<PatientNode> {
        let loader = ctx.get_loader::<DataLoader<PatientLoader>>();

        let result = loader
            .load_one(self.encounter.0.patient_id.clone())
            .await?
            .map(|patient| PatientNode {
                store_id: self.store_id.clone(),
                allowed_ctx: self.allowed_ctx.clone(),
                patient,
            })
            .ok_or(Error::new("Encounter without patient"))?;

        Ok(result)
    }

    pub async fn clinician(&self, ctx: &Context<'_>) -> Result<Option<ClinicianNode>> {
        let Some(clinician_id) = self.encounter.0.clinician_id.as_ref() else {
            return Ok(None)
        };
        let loader = ctx.get_loader::<DataLoader<ClinicianLoader>>();

        let result = loader
            .load_one(ClinicianLoaderInput::new(&self.store_id, &clinician_id))
            .await?
            .map(ClinicianNode::from_domain)
            .ok_or(Error::new(format!(
                "Failed to load clinician: {}",
                clinician_id
            )))?;

        Ok(Some(result))
    }

    /// Returns the matching program enrolment for the patient of this encounter
    pub async fn program_enrolment(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<ProgramEnrolmentNode>> {
        let loader = ctx.get_loader::<DataLoader<ProgramEnrolmentLoader>>();

        let result = loader
            .load_one(ProgramEnrolmentLoaderInput::new(
                &self.encounter.0.patient_id,
                &self.encounter.0.program_id,
                self.allowed_ctx.clone(),
            ))
            .await?
            .map(|program_enrolment| ProgramEnrolmentNode {
                store_id: self.store_id.clone(),
                program_enrolment,
                allowed_ctx: self.allowed_ctx.clone(),
            })
            .ok_or(Error::new(format!(
                "Failed to load program enrolment: {}",
                self.encounter.0.program_id
            )))?;

        Ok(Some(result))
    }

    pub async fn r#type(&self) -> &str {
        &self.encounter.0.document_type
    }

    pub async fn name(&self) -> &str {
        &self.encounter.0.document_name
    }

    pub async fn status(&self) -> Option<EncounterNodeStatus> {
        self.encounter
            .0
            .status
            .as_ref()
            .map(|status| EncounterNodeStatus::from_domain(status))
    }

    pub async fn created_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.encounter.0.created_datetime, Utc)
    }

    pub async fn start_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.encounter.0.start_datetime, Utc)
    }

    pub async fn end_datetime(&self) -> Option<DateTime<Utc>> {
        self.encounter
            .0
            .end_datetime
            .map(|t| DateTime::<Utc>::from_utc(t, Utc))
    }

    /// The encounter document
    pub async fn document(&self, ctx: &Context<'_>) -> Result<DocumentNode> {
        let loader = ctx.get_loader::<DataLoader<DocumentLoader>>();

        let result = loader
            .load_one(self.encounter.0.document_name.clone())
            .await?
            .map(|document| DocumentNode {
                allowed_ctx: self.allowed_ctx.clone(),
                document,
            })
            .ok_or(Error::new("Program without document"))?;

        Ok(result)
    }

    pub async fn active_program_events(
        &self,
        ctx: &Context<'_>,
        at: Option<DateTime<Utc>>,
        filter: Option<ActiveEncounterEventFilterInput>,
    ) -> Result<ProgramEventResponse> {
        // TODO use loader?
        let context = ctx.service_provider().basic_context()?;
        let mut program_filter = filter
            .as_ref()
            .map(|f| f.clone().to_domain())
            .unwrap_or(ProgramEventFilter::new())
            .patient_id(EqualFilter::equal_to(&self.encounter.0.patient_id))
            .document_type(EqualFilter::equal_to(&self.encounter.0.document_type));
        if filter.and_then(|f| f.is_current_encounter).unwrap_or(false) {
            program_filter =
                program_filter.document_name(EqualFilter::equal_to(&self.encounter.0.document_name))
        };
        let list_result = ctx
            .service_provider()
            .program_event_service
            .active_events(
                &context,
                at.map(|at| at.naive_utc())
                    .unwrap_or(Utc::now().naive_utc()),
                None,
                Some(program_filter),
                Some(Sort {
                    key: ProgramEventSortField::Datetime,
                    desc: Some(true),
                }),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(ProgramEventResponse::Response(ProgramEventConnector {
            total_count: list_result.count,
            nodes: list_result
                .rows
                .into_iter()
                .map(|row| ProgramEventNode {
                    store_id: self.store_id.clone(),
                    row,
                    allowed_ctx: self.allowed_ctx.clone(),
                })
                .collect(),
        }))
    }

    pub async fn program_events(
        &self,
        ctx: &Context<'_>,
        page: Option<PaginationInput>,
        sort: Option<ProgramEventSortInput>,
        filter: Option<EncounterEventFilterInput>,
    ) -> Result<ProgramEventResponse> {
        let context = ctx.service_provider().basic_context()?;
        let mut program_filter = filter
            .as_ref()
            .map(|f| f.clone().to_domain())
            .unwrap_or(ProgramEventFilter::new())
            .patient_id(EqualFilter::equal_to(&self.encounter.0.patient_id))
            .document_type(EqualFilter::equal_to(&self.encounter.0.document_type));
        if filter.and_then(|f| f.is_current_encounter).unwrap_or(false) {
            program_filter =
                program_filter.document_name(EqualFilter::equal_to(&self.encounter.0.document_name))
        };
        let list_result = ctx
            .service_provider()
            .program_event_service
            .events(
                &context,
                page.map(PaginationOption::from),
                Some(program_filter),
                sort.map(ProgramEventSortInput::to_domain),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(ProgramEventResponse::Response(ProgramEventConnector {
            total_count: list_result.count,
            nodes: list_result
                .rows
                .into_iter()
                .map(|row| ProgramEventNode {
                    store_id: self.store_id.clone(),
                    row,
                    allowed_ctx: self.allowed_ctx.clone(),
                })
                .collect(),
        }))
    }
}

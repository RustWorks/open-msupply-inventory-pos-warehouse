use super::{
    program_enrolment_row::program_enrolment::{self, dsl as program_enlrolment_dsl},
    program_row::{program, program::dsl as program_dsl},
    StorageConnection,
};

use crate::{
    diesel_macros::{apply_date_time_filter, apply_equal_filter, apply_sort, apply_sort_no_case},
    DBType, DatetimeFilter, EqualFilter, Pagination, ProgramEnrolmentRow, ProgramEnrolmentStatus,
    ProgramRow, RepositoryError, Sort,
};

use diesel::{dsl::IntoBoxed, helper_types::InnerJoin, prelude::*};

#[derive(Clone)]
pub struct ProgramEnrolmentFilter {
    pub patient_id: Option<EqualFilter<String>>,
    pub program_id: Option<EqualFilter<String>>,
    pub enrolment_datetime: Option<DatetimeFilter>,
    pub program_enrolment_id: Option<EqualFilter<String>>,
    pub status: Option<EqualFilter<ProgramEnrolmentStatus>>,
    pub document_type: Option<EqualFilter<String>>,
    pub document_name: Option<EqualFilter<String>>,
    pub program_context_id: Option<EqualFilter<String>>,
}

impl ProgramEnrolmentFilter {
    pub fn new() -> ProgramEnrolmentFilter {
        ProgramEnrolmentFilter {
            patient_id: None,
            program_id: None,
            program_context_id: None,
            enrolment_datetime: None,
            program_enrolment_id: None,
            status: None,
            document_type: None,
            document_name: None,
        }
    }

    pub fn program_id(mut self, filter: EqualFilter<String>) -> Self {
        self.program_id = Some(filter);
        self
    }

    pub fn context_id(mut self, filter: EqualFilter<String>) -> Self {
        self.program_context_id = Some(filter);
        self
    }

    pub fn patient_id(mut self, filter: EqualFilter<String>) -> Self {
        self.patient_id = Some(filter);
        self
    }

    pub fn enrolment_datetime(mut self, filter: DatetimeFilter) -> Self {
        self.enrolment_datetime = Some(filter);
        self
    }

    pub fn program_enrolment_id(mut self, filter: EqualFilter<String>) -> Self {
        self.program_enrolment_id = Some(filter);
        self
    }

    pub fn status(mut self, filter: EqualFilter<ProgramEnrolmentStatus>) -> Self {
        self.status = Some(filter);
        self
    }

    pub fn document_type(mut self, filter: EqualFilter<String>) -> Self {
        self.document_type = Some(filter);
        self
    }

    pub fn document_name(mut self, filter: EqualFilter<String>) -> Self {
        self.document_name = Some(filter);
        self
    }
}

pub enum ProgramEnrolmentSortField {
    Type,
    PatientId,
    EnrolmentDatetime,
    ProgramEnrolmentId,
    Status,
}

pub type ProgramEnrolment = (ProgramEnrolmentRow, ProgramRow);

pub type ProgramEnrolmentSort = Sort<ProgramEnrolmentSortField>;

type BoxedProgramEnrolmentQuery =
    IntoBoxed<'static, InnerJoin<program_enrolment::table, program::table>, DBType>;

fn create_filtered_query<'a>(filter: Option<ProgramEnrolmentFilter>) -> BoxedProgramEnrolmentQuery {
    let mut query = program_enlrolment_dsl::program_enrolment
        .inner_join(program_dsl::program)
        .into_boxed();

    if let Some(ProgramEnrolmentFilter {
        patient_id,
        program_id,
        enrolment_datetime,
        program_enrolment_id,
        status,
        document_type,
        document_name,
        program_context_id: context,
    }) = filter
    {
        apply_equal_filter!(query, patient_id, program_enlrolment_dsl::patient_id);
        apply_equal_filter!(query, program_id, program_enlrolment_dsl::program_id);
        apply_equal_filter!(query, context, program_dsl::context_id);
        apply_date_time_filter!(
            query,
            enrolment_datetime,
            program_enlrolment_dsl::enrolment_datetime
        );
        apply_equal_filter!(
            query,
            program_enrolment_id,
            program_enlrolment_dsl::program_enrolment_id
        );
        apply_equal_filter!(query, status, program_enlrolment_dsl::status);
        apply_equal_filter!(query, document_type, program_enlrolment_dsl::document_type);
        apply_equal_filter!(query, document_name, program_enlrolment_dsl::document_name);
    }
    query
}

pub struct ProgramEnrolmentRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> ProgramEnrolmentRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        ProgramEnrolmentRepository { connection }
    }

    pub fn count(&self, filter: Option<ProgramEnrolmentFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: ProgramEnrolmentFilter,
    ) -> Result<Vec<ProgramEnrolment>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<ProgramEnrolmentFilter>,
        sort: Option<ProgramEnrolmentSort>,
    ) -> Result<Vec<ProgramEnrolment>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                ProgramEnrolmentSortField::PatientId => {
                    apply_sort!(query, sort, program_enlrolment_dsl::patient_id)
                }
                ProgramEnrolmentSortField::Type => {
                    apply_sort!(query, sort, program_enlrolment_dsl::document_type)
                }
                ProgramEnrolmentSortField::EnrolmentDatetime => {
                    apply_sort!(query, sort, program_enlrolment_dsl::enrolment_datetime)
                }
                ProgramEnrolmentSortField::ProgramEnrolmentId => {
                    apply_sort!(query, sort, program_enlrolment_dsl::program_enrolment_id)
                }
                ProgramEnrolmentSortField::Status => {
                    apply_sort_no_case!(query, sort, program_enlrolment_dsl::status)
                }
            }
        } else {
            query = query.order(program_enlrolment_dsl::document_type.asc())
        }

        let result = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64)
            .load::<ProgramEnrolment>(&self.connection.connection)?;

        Ok(result)
    }

    pub fn find_one_by_type_and_patient(
        &self,
        r#type: &str,
        patient_id: &str,
    ) -> Result<Option<ProgramEnrolment>, RepositoryError> {
        Ok(self
            .query_by_filter(
                ProgramEnrolmentFilter::new()
                    .document_type(EqualFilter::equal_to(r#type))
                    .patient_id(EqualFilter::equal_to(patient_id)),
            )?
            .pop())
    }
}

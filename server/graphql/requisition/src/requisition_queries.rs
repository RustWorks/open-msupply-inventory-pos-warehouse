use async_graphql::*;
use repository::{DatetimeFilter, EqualFilter, PaginationOption, SimpleStringFilter};
use graphql_core::{
    generic_filters::{
        DatetimeFilterInput, EqualFilterBigNumberInput, EqualFilterStringInput,
        SimpleStringFilterInput,
    },
    pagination::PaginationInput,
    simple_generic_errors::RecordNotFound,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::{
    RequisitionConnector, RequisitionNode, RequisitionNodeStatus, RequisitionNodeType,
};
use repository::{RequisitionFilter, RequisitionSort, RequisitionSortField};
use service::permission_validation::{Resource, ResourceAccessRequest};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum RequisitionSortFieldInput {
    RequisitionNumber,
    Type,
    Status,
    OtherPartyName,
    SentDatetime,
    CreatedDatetime,
    FinalisedDatetime,
}

#[derive(InputObject)]
pub struct RequisitionSortInput {
    /// Sort query result by `key`
    key: RequisitionSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterRequisitionTypeInput {
    pub equal_to: Option<RequisitionNodeType>,
    pub equal_any: Option<Vec<RequisitionNodeType>>,
    pub not_equal_to: Option<RequisitionNodeType>,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterRequisitionStatusInput {
    pub equal_to: Option<RequisitionNodeStatus>,
    pub equal_any: Option<Vec<RequisitionNodeStatus>>,
    pub not_equal_to: Option<RequisitionNodeStatus>,
}

#[derive(InputObject, Clone)]
pub struct RequisitionFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub requisition_number: Option<EqualFilterBigNumberInput>,
    pub r#type: Option<EqualFilterRequisitionTypeInput>,
    pub status: Option<EqualFilterRequisitionStatusInput>,
    pub created_datetime: Option<DatetimeFilterInput>,
    pub sent_datetime: Option<DatetimeFilterInput>,
    pub finalised_datetime: Option<DatetimeFilterInput>,
    pub other_party_name: Option<SimpleStringFilterInput>,
    pub other_party_id: Option<EqualFilterStringInput>,
    pub colour: Option<EqualFilterStringInput>,
    pub their_reference: Option<SimpleStringFilterInput>,
    pub comment: Option<SimpleStringFilterInput>,
}

#[derive(Union)]
pub enum RequisitionsResponse {
    Response(RequisitionConnector),
}

#[derive(Union)]
pub enum RequisitionResponse {
    Error(RecordNotFound),
    Response(RequisitionNode),
}

pub fn get_requisition(ctx: &Context<'_>, store_id: &str, id: &str) -> Result<RequisitionResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryRequisition,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.context()?;

    let requisition_option = service_provider.requisition_service.get_requisition(
        &service_context,
        Some(store_id),
        id,
    )?;

    let response = match requisition_option {
        Some(requisition) => {
            RequisitionResponse::Response(RequisitionNode::from_domain(requisition))
        }
        None => RequisitionResponse::Error(RecordNotFound {}),
    };

    Ok(response)
}

pub fn get_requisitions(
    ctx: &Context<'_>,
    store_id: &str,
    page: Option<PaginationInput>,
    filter: Option<RequisitionFilterInput>,
    sort: Option<Vec<RequisitionSortInput>>,
) -> Result<RequisitionsResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryRequisition,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.context()?;

    let requisitions = service_provider
        .requisition_service
        .get_requisitions(
            &service_context,
            Some(&store_id),
            page.map(PaginationOption::from),
            filter.map(|filter| filter.to_domain()),
            // Currently only one sort option is supported, use the first from the list.
            sort.map(|mut sort_list| sort_list.pop())
                .flatten()
                .map(|sort| sort.to_domain()),
        )
        .map_err(StandardGraphqlError::from_list_error)?;

    Ok(RequisitionsResponse::Response(
        RequisitionConnector::from_domain(requisitions),
    ))
}

pub fn get_requisition_by_number(
    ctx: &Context<'_>,
    store_id: &str,
    requisition_number: u32,
    r#type: RequisitionNodeType,
) -> Result<RequisitionResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryRequisition,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.context()?;

    let requisition_option = service_provider
        .requisition_service
        .get_requisition_by_number(
            &service_context,
            store_id,
            requisition_number,
            r#type.to_domain(),
        )?;

    let response = match requisition_option {
        Some(requisition) => {
            RequisitionResponse::Response(RequisitionNode::from_domain(requisition))
        }
        None => RequisitionResponse::Error(RecordNotFound {}),
    };

    Ok(response)
}

impl RequisitionSortInput {
    pub fn to_domain(self) -> RequisitionSort {
        use RequisitionSortField as to;
        use RequisitionSortFieldInput as from;
        let key = match self.key {
            from::RequisitionNumber => to::RequisitionNumber,
            from::Type => to::Type,
            from::Status => to::Status,
            from::OtherPartyName => to::OtherPartyName,
            from::SentDatetime => to::SentDatetime,
            from::CreatedDatetime => to::CreatedDatetime,
            from::FinalisedDatetime => to::FinalisedDatetime,
        };

        RequisitionSort {
            key,
            desc: self.desc,
        }
    }
}

macro_rules! map_filter {
    ($from:ident, $f:expr) => {{
        EqualFilter {
            equal_to: $from.equal_to.map($f),
            not_equal_to: $from.not_equal_to.map($f),
            equal_any: $from
                .equal_any
                .map(|inputs| inputs.into_iter().map($f).collect()),
            not_equal_all: None,
        }
    }};
}

impl RequisitionFilterInput {
    pub fn to_domain(self) -> RequisitionFilter {
        RequisitionFilter {
            id: self.id.map(EqualFilter::from),
            requisition_number: self.requisition_number.map(EqualFilter::from),
            r#type: self
                .r#type
                .map(|t| map_filter!(t, RequisitionNodeType::to_domain)),
            status: self
                .status
                .map(|t| map_filter!(t, RequisitionNodeStatus::to_domain)),
            created_datetime: self.created_datetime.map(DatetimeFilter::from),
            sent_datetime: self.sent_datetime.map(DatetimeFilter::from),
            finalised_datetime: self.finalised_datetime.map(DatetimeFilter::from),
            name: self.other_party_name.map(SimpleStringFilter::from),
            name_id: self.other_party_id.map(EqualFilter::from),
            colour: self.colour.map(EqualFilter::from),
            their_reference: self.their_reference.map(SimpleStringFilter::from),
            comment: self.comment.map(SimpleStringFilter::from),
            linked_requisition_id: None,
            store_id: None,
        }
    }
}

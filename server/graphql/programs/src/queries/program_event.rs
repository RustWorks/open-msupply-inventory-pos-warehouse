use async_graphql::*;
use chrono::{DateTime, Utc};
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::{
    program_enrolment::ProgramEventFilterInput,
    program_event::{
        ProgramEventConnector, ProgramEventNode, ProgramEventResponse, ProgramEventSortInput,
    },
};
use repository::{PaginationOption, ProgramEventFilter};
use service::auth::{Resource, ResourceAccessRequest};

pub fn program_events(
    ctx: &Context<'_>,
    store_id: String,
    page: Option<PaginationInput>,
    sort: Option<ProgramEventSortInput>,
    filter: Option<ProgramEventFilterInput>,
) -> Result<ProgramEventResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryProgram,
            store_id: Some(store_id.clone()),
        },
    )?;
    let allowed_ctx = user.capabilities();

    let mut filter = filter
        .map(|f| f.to_domain())
        .unwrap_or(ProgramEventFilter::new());
    // restrict query results to allowed entries
    filter.context_id = Some(
        filter
            .context_id
            .unwrap_or_default()
            .restrict_results(&allowed_ctx),
    );

    let service_provider = ctx.service_provider();
    let context = service_provider.basic_context()?;

    let list_result = service_provider
        .program_event_service
        .events(
            &context,
            page.map(PaginationOption::from),
            Some(filter),
            sort.map(ProgramEventSortInput::to_domain),
        )
        .map_err(StandardGraphqlError::from_list_error)?;
    let nodes: Vec<ProgramEventNode> = list_result
        .rows
        .into_iter()
        .map(|row| ProgramEventNode {
            store_id: store_id.clone(),
            row,
            allowed_ctx: allowed_ctx.clone(),
        })
        .collect();

    Ok(ProgramEventResponse::Response(ProgramEventConnector {
        total_count: list_result.count,
        nodes,
    }))
}

pub fn active_program_events(
    ctx: &Context<'_>,
    store_id: String,
    at: Option<DateTime<Utc>>,
    page: Option<PaginationInput>,
    sort: Option<ProgramEventSortInput>,
    filter: Option<ProgramEventFilterInput>,
) -> Result<ProgramEventResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryProgram,
            store_id: Some(store_id.clone()),
        },
    )?;
    let allowed_ctx = user.capabilities();

    let mut filter = filter
        .map(|f| f.to_domain())
        .unwrap_or(ProgramEventFilter::new());
    // restrict query results to allowed entries
    filter.context_id = Some(
        filter
            .context_id
            .unwrap_or_default()
            .restrict_results(&allowed_ctx),
    );

    let service_provider = ctx.service_provider();
    let context = service_provider.basic_context()?;

    let list_result = service_provider
        .program_event_service
        .active_events(
            &context,
            at.map(|at| at.naive_utc())
                .unwrap_or(Utc::now().naive_utc()),
            page.map(PaginationOption::from),
            Some(filter),
            sort.map(ProgramEventSortInput::to_domain),
        )
        .map_err(StandardGraphqlError::from_list_error)?;
    let nodes: Vec<ProgramEventNode> = list_result
        .rows
        .into_iter()
        .map(|row| ProgramEventNode {
            store_id: store_id.clone(),
            row,
            allowed_ctx: allowed_ctx.clone(),
        })
        .collect();

    Ok(ProgramEventResponse::Response(ProgramEventConnector {
        total_count: list_result.count,

        nodes,
    }))
}

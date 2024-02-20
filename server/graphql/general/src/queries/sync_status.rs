pub use async_graphql::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use service::{
    auth::{Resource, ResourceAccessRequest},
    sync::sync_status::status::FullSyncStatus,
};

use crate::sync_api_error::SyncErrorNode;

pub struct SyncStatusNode {
    started: NaiveDateTime,
    finished: Option<NaiveDateTime>,
}

#[Object]
impl SyncStatusNode {
    async fn started(&self) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(self.started, Utc)
    }

    async fn finished(&self) -> Option<DateTime<Utc>> {
        self.finished
            .map(|v| DateTime::from_naive_utc_and_offset(v, Utc))
    }
}

pub struct SyncStatusWithProgressNode {
    started: NaiveDateTime,
    finished: Option<NaiveDateTime>,
    total: Option<u32>,
    done: Option<u32>,
}

#[Object]
impl SyncStatusWithProgressNode {
    async fn started(&self) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(self.started, Utc)
    }

    async fn finished(&self) -> Option<DateTime<Utc>> {
        self.finished
            .map(|v| DateTime::from_naive_utc_and_offset(v, Utc))
    }

    async fn total(&self) -> &Option<u32> {
        &self.total
    }

    async fn done(&self) -> &Option<u32> {
        &self.done
    }
}

#[derive(SimpleObject)]
pub struct FullSyncStatusNode {
    is_syncing: bool,
    error: Option<SyncErrorNode>,
    summary: SyncStatusNode,
    prepare_initial: Option<SyncStatusNode>,
    integration: Option<SyncStatusWithProgressNode>,
    pull_central: Option<SyncStatusWithProgressNode>,
    pull_remote: Option<SyncStatusWithProgressNode>,
    push: Option<SyncStatusWithProgressNode>,
    last_successful_sync: Option<SyncStatusNode>,
}

pub fn latest_sync_status(
    ctx: &Context<'_>,
    with_auth: bool,
) -> Result<Option<FullSyncStatusNode>> {
    if with_auth {
        validate_sync_info_auth(ctx)?
    };

    let service_provider = ctx.service_provider();
    let ctx = service_provider.basic_context()?;
    let sync_status = match service_provider
        .sync_status_service
        .get_latest_sync_status(&ctx)?
    {
        Some(sync_status) => sync_status,
        None => return Ok(None),
    };
    let last_successful_sync_status = service_provider
        .sync_status_service
        .get_latest_successful_sync_status(&ctx)
        .unwrap_or(None);

    let FullSyncStatus {
        is_syncing,
        error,
        summary,
        prepare_initial,
        integration,
        pull_central,
        pull_remote,
        push,
    } = sync_status;

    let result = FullSyncStatusNode {
        is_syncing,
        error: error.map(SyncErrorNode::from_sync_log_error),
        summary: SyncStatusNode {
            started: summary.started,
            finished: summary.finished,
        },
        prepare_initial: prepare_initial.map(|status| SyncStatusNode {
            started: status.started,
            finished: status.finished,
        }),
        integration: integration.map(|status| SyncStatusWithProgressNode {
            started: status.started,
            finished: status.finished,
            total: status.total,
            done: status.done,
        }),
        pull_central: pull_central.map(|status| SyncStatusWithProgressNode {
            started: status.started,
            finished: status.finished,
            total: status.total,
            done: status.done,
        }),
        pull_remote: pull_remote.map(|status| SyncStatusWithProgressNode {
            started: status.started,
            finished: status.finished,
            total: status.total,
            done: status.done,
        }),
        push: push.map(|status| SyncStatusWithProgressNode {
            started: status.started,
            finished: status.finished,
            total: status.total,
            done: status.done,
        }),
        last_successful_sync: match last_successful_sync_status {
            None => None,
            Some(last_successful_sync_status) => Some(SyncStatusNode {
                started: last_successful_sync_status.summary.started,
                finished: last_successful_sync_status.summary.finished,
            }),
        },
    };

    Ok(Some(result))
}

pub fn number_of_records_in_push_queue(ctx: &Context<'_>) -> Result<u64> {
    validate_sync_info_auth(ctx)?;

    let service_provider = ctx.service_provider();
    let ctx = service_provider.basic_context()?;
    let push_queue_count = service_provider
        .sync_status_service
        .number_of_records_in_push_queue(&ctx)
        .map_err(|error| {
            let formatted_error = format!("{:#?}", error);
            StandardGraphqlError::InternalError(formatted_error).extend()
        })?;

    Ok(push_queue_count)
}

fn validate_sync_info_auth(ctx: &Context<'_>) -> Result<()> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::SyncInfo,
            store_id: None,
        },
    )?;

    Ok(())
}

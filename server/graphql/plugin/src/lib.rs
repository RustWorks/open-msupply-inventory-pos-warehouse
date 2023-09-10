pub mod plugin_data;
use self::plugin_data::mutations;

use async_graphql::*;
use graphql_types::types::{PluginDataConnector, RelatedRecordNodeType};
use plugin_data::query::{PluginDataFilterInput, PluginDataSortInput};

#[derive(Default, Clone)]
pub struct PluginQueries;

#[Object]
impl PluginQueries {
    async fn plugin_data(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        r#type: RelatedRecordNodeType,
        filter: Option<PluginDataFilterInput>,
        sort: Option<Vec<PluginDataSortInput>>,
    ) -> Result<PluginDataConnector> {
        plugin_data::query::get_plugin_data(ctx, &store_id, r#type, filter, sort)
    }
}

#[derive(Default, Clone)]
pub struct PluginMutations;

#[Object]
impl PluginMutations {
    async fn insert_plugin_data(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: mutations::insert::InsertPluginDataInput,
    ) -> Result<mutations::insert::InsertResponse> {
        mutations::insert::insert_plugin_data(ctx, &store_id, input)
    }

    async fn update_plugin_data(
        &self,
        ctx: &Context<'_>,
        store_id: String,
        input: mutations::update::UpdatePluginDataInput,
    ) -> Result<mutations::update::UpdateResponse> {
        mutations::update::update_plugin_data(ctx, &store_id, input)
    }
}

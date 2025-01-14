use std::vec;

use async_graphql::dataloader::DataLoader;
use async_graphql::*;
use graphql_asset_catalogue::types::asset_catalogue_item::AssetCatalogueItemNode;
use graphql_asset_catalogue::types::asset_category::AssetCategoryNode;
use graphql_asset_catalogue::types::asset_class::AssetClassNode;
use graphql_asset_catalogue::types::asset_type::AssetTypeNode;
use graphql_core::generic_filters::{DateFilterInput, EqualFilterStringInput, StringFilterInput};
use graphql_core::loader::AssetStatusLogLoader;
use graphql_core::loader::SyncFileReferenceLoader;
use graphql_core::loader::{
    AssetCatalogueItemLoader, AssetCatalogueItemPropertyLoader, AssetCategoryLoader,
    AssetClassLoader, AssetLocationLoader, AssetTypeLoader, StoreByIdLoader,
};
use graphql_core::simple_generic_errors::NodeError;
use graphql_core::{map_filter, ContextExt};
use graphql_types::types::{LocationConnector, StoreNode, SyncFileReferenceConnector};

use repository::asset_catalogue_item_property::AssetCatalogueItemPropertyValue;
use repository::assets::asset::AssetSortField;

use repository::{
    assets::asset::{Asset, AssetFilter, AssetSort},
    EqualFilter,
};
use repository::{DateFilter, StringFilter};
use service::{usize_to_u32, ListResult};

use super::{
    AssetCatalogueItemPropertyValueNode, AssetLogNode, AssetLogStatusInput, EqualFilterStatusInput,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum AssetSortFieldInput {
    SerialNumber,
    InstallationDate,
    ReplacementDate,
    ModifiedDatetime,
    AssetNumber,
    Store,
}

#[derive(InputObject)]
pub struct AssetSortInput {
    /// Sort query result by `key`
    key: AssetSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}

#[derive(InputObject, Clone)]
pub struct AssetFilterInput {
    pub notes: Option<StringFilterInput>,
    pub asset_number: Option<StringFilterInput>,
    pub id: Option<EqualFilterStringInput>,
    pub serial_number: Option<StringFilterInput>,
    pub class_id: Option<EqualFilterStringInput>,
    pub category_id: Option<EqualFilterStringInput>,
    pub type_id: Option<EqualFilterStringInput>,
    pub catalogue_item_id: Option<EqualFilterStringInput>,
    pub is_non_catalogue: Option<bool>,
    pub installation_date: Option<DateFilterInput>,
    pub replacement_date: Option<DateFilterInput>,
    pub store: Option<StringFilterInput>,
    pub functional_status: Option<EqualFilterStatusInput>,
}

impl From<AssetFilterInput> for AssetFilter {
    fn from(f: AssetFilterInput) -> Self {
        AssetFilter {
            notes: f.notes.map(StringFilter::from),
            asset_number: f.asset_number.map(StringFilter::from),
            id: f.id.map(EqualFilter::from),
            serial_number: f.serial_number.map(StringFilter::from),
            class_id: f.class_id.map(EqualFilter::from),
            category_id: f.category_id.map(EqualFilter::from),
            type_id: f.type_id.map(EqualFilter::from),
            catalogue_item_id: f.catalogue_item_id.map(EqualFilter::from),
            installation_date: f.installation_date.map(DateFilter::from),
            replacement_date: f.replacement_date.map(DateFilter::from),
            is_non_catalogue: f.is_non_catalogue,
            store: f.store.map(StringFilter::from),
            functional_status: f
                .functional_status
                .map(|t| map_filter!(t, AssetLogStatusInput::to_domain)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AssetNode {
    pub asset: Asset,
}

#[derive(SimpleObject)]
pub struct AssetConnector {
    total_count: u32,
    nodes: Vec<AssetNode>,
}

impl AssetConnector {
    pub fn new() -> AssetConnector {
        AssetConnector {
            total_count: 0,
            nodes: Vec::<AssetNode>::new(),
        }
    }
}

#[Object]
impl AssetNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }

    pub async fn store_id(&self) -> &Option<String> {
        &self.row().store_id
    }

    pub async fn notes(&self) -> &Option<String> {
        &self.row().notes
    }

    pub async fn asset_number(&self) -> &Option<String> {
        &self.row().asset_number
    }

    pub async fn serial_number(&self) -> &Option<String> {
        &self.row().serial_number
    }

    pub async fn catalogue_item_id(&self) -> &Option<String> {
        &self.row().catalogue_item_id
    }

    pub async fn installation_date(&self) -> &Option<chrono::NaiveDate> {
        &self.row().installation_date
    }

    pub async fn replacement_date(&self) -> &Option<chrono::NaiveDate> {
        &self.row().replacement_date
    }

    pub async fn created_datetime(&self) -> &chrono::NaiveDateTime {
        &self.row().created_datetime
    }

    pub async fn modified_datetime(&self) -> &chrono::NaiveDateTime {
        &self.row().modified_datetime
    }

    pub async fn store(&self, ctx: &Context<'_>) -> Result<Option<StoreNode>> {
        let store_id = match &self.row().store_id {
            Some(store_id) => store_id,
            None => return Ok(None),
        };

        let loader = ctx.get_loader::<DataLoader<StoreByIdLoader>>();
        Ok(loader
            .load_one(store_id.clone())
            .await?
            .map(StoreNode::from_domain))
    }

    pub async fn catalogue_item(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<AssetCatalogueItemNode>> {
        let catalogue_item_id = match &self.row().catalogue_item_id {
            Some(catalogue_item_id) => catalogue_item_id,
            None => return Ok(None),
        };

        let loader = ctx.get_loader::<DataLoader<AssetCatalogueItemLoader>>();
        Ok(loader
            .load_one(catalogue_item_id.clone())
            .await?
            .map(AssetCatalogueItemNode::from_domain))
    }

    pub async fn locations(&self, ctx: &Context<'_>) -> Result<LocationConnector> {
        let asset_id = &self.row().id;
        let loader = ctx.get_loader::<DataLoader<AssetLocationLoader>>();
        let result_option = loader.load_one(asset_id.to_string()).await?;

        let locations = LocationConnector::from_vec(result_option.unwrap_or(vec![]));

        Ok(locations)
    }

    pub async fn documents(&self, ctx: &Context<'_>) -> Result<SyncFileReferenceConnector> {
        let asset_id = &self.row().id;
        let loader = ctx.get_loader::<DataLoader<SyncFileReferenceLoader>>();
        let result_option = loader.load_one(asset_id.to_string()).await?;

        let documents = SyncFileReferenceConnector::from_vec(result_option.unwrap_or(vec![]));

        Ok(documents)
    }

    pub async fn catalog_properties(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<AssetCatalogueItemPropertyValueNode>> {
        let properties = match &self.row().catalogue_item_id {
            Some(catalogue_item_id) => {
                let loader = ctx.get_loader::<DataLoader<AssetCatalogueItemPropertyLoader>>();
                let result_option = loader.load_one(catalogue_item_id.to_string()).await?;

                result_option
                    .unwrap_or(Vec::<AssetCatalogueItemPropertyValue>::new())
                    .iter()
                    .map(|p| AssetCatalogueItemPropertyValueNode::from_domain(p.to_owned()))
                    .into_iter()
                    .collect()
            }
            None => vec![],
        };

        Ok(properties)
    }

    pub async fn properties(&self) -> Result<String> {
        let asset_properties = match &self.row().properties {
            Some(properties) => properties.to_owned(),
            None => return Ok("{}".to_string()), // Empty JSON object
        };
        Ok(asset_properties)
    }

    pub async fn asset_category(&self, ctx: &Context<'_>) -> Result<Option<AssetCategoryNode>> {
        let loader = ctx.get_loader::<DataLoader<AssetCategoryLoader>>();
        let category_id = match self.row().asset_category_id.clone() {
            Some(category_id) => category_id,
            None => return Ok(None),
        };

        Ok(loader
            .load_one(category_id)
            .await?
            .map(AssetCategoryNode::from_domain))
    }

    pub async fn asset_class(&self, ctx: &Context<'_>) -> Result<Option<AssetClassNode>> {
        let loader = ctx.get_loader::<DataLoader<AssetClassLoader>>();
        let class_id = match self.row().asset_class_id.clone() {
            Some(class_id) => class_id,
            None => return Ok(None),
        };

        Ok(loader
            .load_one(class_id)
            .await?
            .map(AssetClassNode::from_domain))
    }

    pub async fn asset_type(&self, ctx: &Context<'_>) -> Result<Option<AssetTypeNode>> {
        let loader = ctx.get_loader::<DataLoader<AssetTypeLoader>>();
        let type_id = match self.row().asset_type_id.clone() {
            Some(type_id) => type_id,
            None => return Ok(None),
        };

        Ok(loader
            .load_one(type_id)
            .await?
            .map(AssetTypeNode::from_domain))
    }

    pub async fn status_log(&self, ctx: &Context<'_>) -> Result<Option<AssetLogNode>> {
        let asset_id = self.row().id.clone();
        let loader = ctx.get_loader::<DataLoader<AssetStatusLogLoader>>();

        Ok(loader
            .load_one(asset_id.clone())
            .await?
            .map(AssetLogNode::from_domain))
    }
}

#[derive(Union)]
pub enum AssetsResponse {
    Response(AssetConnector),
}

#[derive(Union)]
pub enum AssetResponse {
    Error(NodeError),
    Response(AssetNode),
}

impl AssetNode {
    pub fn from_domain(asset: Asset) -> AssetNode {
        AssetNode { asset }
    }

    pub fn row(&self) -> &Asset {
        &self.asset
    }
}

impl AssetConnector {
    pub fn from_domain(assets: ListResult<Asset>) -> AssetConnector {
        AssetConnector {
            total_count: assets.count,
            nodes: assets
                .rows
                .into_iter()
                .map(AssetNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(assets: Vec<Asset>) -> AssetConnector {
        AssetConnector {
            total_count: usize_to_u32(assets.len()),
            nodes: assets.into_iter().map(AssetNode::from_domain).collect(),
        }
    }
}

impl AssetSortInput {
    pub fn to_domain(&self) -> AssetSort {
        use AssetSortField as to;
        use AssetSortFieldInput as from;
        let key = match self.key {
            from::SerialNumber => to::SerialNumber,
            from::InstallationDate => to::InstallationDate,
            from::ReplacementDate => to::ReplacementDate,
            from::ModifiedDatetime => to::ModifiedDatetime,
            from::AssetNumber => to::AssetNumber,
            from::Store => to::Store,
        };

        AssetSort {
            key,
            desc: self.desc,
        }
    }
}

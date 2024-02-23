use async_graphql::*;
use graphql_core::{
    generic_filters::{EqualFilterStringInput, StringFilterInput},
    map_filter,
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::{AssetCatalogueItemConnector, AssetCatalogueItemNode};
use repository::asset_catalogue_item::{
    AssetCatalogueItemFilter, AssetCatalogueItemSort, AssetCatalogueItemSortField,
};
use repository::{EqualFilter, PaginationOption, StringFilter};
use service::{
    auth::{Resource, ResourceAccessRequest},
    catalogue::query_catalogue_item::get_asset_catalogue_items,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(remote = "repository::asset_catalogue_item::AssetCatalogueItemSortField")]
#[graphql(rename_items = "camelCase")]

pub enum AssetCatalogueItemSortFieldInput {
    Catalogue,
    Code,
    Make,
    Model,
}

#[derive(InputObject)]

pub struct AssetCatalogueItemSortInput {
    key: AssetCatalogueItemSortFieldInput,
    desc: Option<bool>,
}

pub struct AssetCatalogueItemFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub category: Option<StringFilterInput>,
    pub category_id: Option<EqualFilterStringInput>,
    pub class: Option<StringFilterInput>,
    pub class_id: Option<EqualFilterStringInput>,
    pub code: Option<StringFilterInput>,
    pub manufacturer: Option<StringFilterInput>,
    pub model: Option<StringFilterInput>,
    pub r#type: Option<StringFilterInput>,
    pub type_id: Option<EqualFilterStringInput>,
}

impl From<AssetCatalogueItemFilterInput> for AssetCatalogueItemFilter {
    fn from(f: AssetCatalogueItemFilterInput) -> Self {
        AssetCatalogueItemFilter {
            id: f.id.map(EqualFilter::from),
            category_id: f.category_id.map(EqualFilter::from),
            category: f.category.map(StringFilter::from),
            class: f.class.map(StringFilter::from),
            class_id: f.class_id.map(EqualFilter::from),
            code: f.code.map(StringFilter::from),
            manufacturer: f.manufacturer.map(StringFilter::from),
            model: f.model.map(StringFilter::from),
            r#type: f.r#type.map(StringFilter::from),
            type_id: f.type_id.map(EqualFilter::from),
        }
    }
}

#[derive(Union)]

pub enum AssetCatalogueItemsResponse {
    Response(AssetCatalogueItemConnector),
}

pub fn asset_catalogue_items(
    ctx: &Context<'_>,
    store_id: String,
    page: Option<PaginationInput>,
    filter: Option<AssetCatalogueItemFilterInput>,
    sort: Option<Vec<AssetCatalogueItemSortInput>>,
) -> Result<AssetCatalogueItemsResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryAssetCatalogueItem,
            store_id: Some(store_id.clone()),
        },
    )?;
    let connection_manager = ctx.get_connection_manager().connection()?;
    let items = get_asset_catalogue_items(
        &connection_manager,
        page.map(PaginationOption::from),
        filter.map(|filter| filter.to_domain()),
        sort.and_then(|mut sort_list| sort_list.pop())
            .map(|sort| sort.to_domain()),
    )
    .map_err(StandardGraphqlError::from_list_error)?;

    Ok(AssetCatalogueItemsResponse::Response(
        AssetCatalogueItemConnector::from_domain(items),
    ))
}

impl AssetCatalogueItemFilterInput {
    pub fn to_domain(self) -> AssetCatalogueItemFilter {
        let AssetCatalogueItemFilterInput {
            id,
            category,
            category_id,
            class,
            class_id,
            code,
            manufacturer,
            model,
            r#type,
            type_id,
        } = self;

        AssetCatalogueItemFilter {
            id: id.map(EqualFilter::from),
            category: category.map(StringFilter::from),
            category_id: category_id.map(EqualFilter::from),
            class: class.map(StringFilter::from),
            class_id: class_id.map(EqualFilter::from),
            code: code.map(StringFilter::from),
            manufacturer: manufacturer.map(StringFilter::from),
            model: model.map(StringFilter::from),
            r#type: r#type.map(|t| map_filter!(t, AssetCatalogueItemNode::from_domain)),
            type_id: type_id.map(EqualFilter::from),
        }
    }
}

impl AssetCatalogueItemSortInput {
    pub fn to_domain(self) -> AssetCatalogueItemSort {
        use AssetCatalogueItemSortField as to;
        use AssetCatalogueItemSortFieldInput as from;
        let key = match self.key {
            from::Catalogue => to::Catalogue,
            from::Code => to::Code,
            from::Make => to::Make,
            from::Model => to::Model,
        };

        AssetCatalogueItemSort {
            key,
            desc: self.desc,
        }
    }
}

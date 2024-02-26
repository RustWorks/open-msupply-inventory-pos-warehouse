use async_graphql::*;
use graphql_core::pagination::PaginationInput;
use graphql_types::types::*;

pub mod asset_catalogue_item_queries;
use self::asset_catalogue_item_queries::*;
pub mod asset_class_queries;
use self::asset_class_queries::*;
pub mod asset_category_queries;
use self::asset_category_queries::*;
pub mod asset_type_queries;
use self::asset_type_queries::*;

#[derive(Default, Clone)]
pub struct AssetCatalogueItemQueries;
#[Object]
impl AssetCatalogueItemQueries {
    // asset catalogue item queries
    pub async fn asset_catalogue_items(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<AssetCatalogueItemFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<AssetCatalogueItemSortInput>>,
    ) -> Result<AssetCatalogueItemsResponse> {
        asset_catalogue_items(ctx, page, filter, sort)
    }

    pub async fn asset_catalogue_item(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the asset catalogue item")] id: String,
    ) -> Result<AssetCatalogueItemResponse> {
        asset_catalogue_item(ctx, id)
    }

    // asset class queries
    pub async fn asset_classes(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<AssetClassFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<AssetClassSortInput>>,
    ) -> Result<AssetClassesResponse> {
        asset_classes(ctx, page, filter, sort)
    }

    pub async fn asset_class(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the class")] id: String,
    ) -> Result<AssetClassResponse> {
        asset_class(ctx, id)
    }

    // asset category queries
    pub async fn asset_categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<AssetCategoryFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<AssetCategorySortInput>>,
    ) -> Result<AssetCategoriesResponse> {
        asset_categories(ctx, page, filter, sort)
    }

    pub async fn asset_category(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the class")] id: String,
    ) -> Result<AssetCategoryResponse> {
        asset_category(ctx, id)
    }

    // asset type queries
    pub async fn asset_types(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<AssetTypeFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<AssetTypeSortInput>>,
    ) -> Result<AssetTypesResponse> {
        asset_types(ctx, page, filter, sort)
    }

    pub async fn asset_type(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the class")] id: String,
    ) -> Result<AssetTypeResponse> {
        asset_type(ctx, id)
    }
}

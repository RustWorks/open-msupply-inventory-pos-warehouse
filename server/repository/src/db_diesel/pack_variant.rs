use super::{
    pack_variant_row::{pack_variant, pack_variant::dsl as pack_variant_dsl},
    DBType, StorageConnection,
};

use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case},
    repository_error::RepositoryError,
    PackVariantRow,
};

use crate::{EqualFilter, Pagination, Sort};

use diesel::prelude::*;

pub type PackVariant = PackVariantRow;

pub struct PackVariantRepository<'a> {
    connection: &'a StorageConnection,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackVariantFilter {
    pub id: Option<EqualFilter<String>>,
    pub item_id: Option<EqualFilter<String>>,
    pub pack_size: Option<EqualFilter<i32>>,
}

pub enum PackVariantSortField {
    PackSize,
}

pub type PackVariantSort = Sort<PackVariantSortField>;
type BoxedPackVariantQuery = pack_variant::BoxedQuery<'static, DBType>;

impl<'a> PackVariantRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        PackVariantRepository { connection }
    }

    pub fn count(&self, filter: Option<PackVariantFilter>) -> Result<i64, RepositoryError> {
        let query = Self::create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: PackVariantFilter,
    ) -> Result<Vec<PackVariant>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn create_filtered_query(filter: Option<PackVariantFilter>) -> BoxedPackVariantQuery {
        let mut query = pack_variant_dsl::pack_variant.into_boxed();

        if let Some(f) = filter {
            apply_equal_filter!(query, f.id, pack_variant_dsl::id);
            apply_equal_filter!(query, f.item_id, pack_variant_dsl::item_id);
            apply_equal_filter!(query, f.pack_size, pack_variant_dsl::pack_size);
        }

        query
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<PackVariantFilter>,
        sort: Option<PackVariantSort>,
    ) -> Result<Vec<PackVariant>, RepositoryError> {
        let mut query = Self::create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                PackVariantSortField::PackSize => {
                    apply_sort_no_case!(query, sort, pack_variant_dsl::pack_size);
                }
            }
        } else {
            query = query.order(pack_variant_dsl::pack_size.asc())
        }

        let result = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64)
            .load::<PackVariant>(&self.connection.connection)?;

        Ok(result)
    }
}

impl PackVariantFilter {
    pub fn new() -> PackVariantFilter {
        PackVariantFilter {
            id: None,
            item_id: None,
            pack_size: None,
        }
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn item_id(mut self, filter: EqualFilter<String>) -> Self {
        self.item_id = Some(filter);
        self
    }

    pub fn pack_size(mut self, filter: EqualFilter<i32>) -> Self {
        self.pack_size = Some(filter);
        self
    }
}

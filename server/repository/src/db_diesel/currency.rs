use super::{
    currency_row::{currency, currency::dsl as currency_dsl},
    CurrencyRow, DBType, StorageConnection,
};
use diesel::prelude::*;

use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case},
    repository_error::RepositoryError,
};

use crate::{EqualFilter, Sort};

#[derive(PartialEq, Debug, Clone)]
pub struct Currency {
    pub currency_row: CurrencyRow,
}

#[derive(Clone)]
pub struct CurrencyFilter {
    pub id: Option<EqualFilter<String>>,
    pub is_home_currency: Option<bool>,
    pub is_active: Option<bool>,
}

pub enum CurrencySortField {
    Id,
    CurrencyCode,
}

pub type CurrencySort = Sort<CurrencySortField>;

pub struct CurrencyRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> CurrencyRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        CurrencyRepository { connection }
    }

    pub fn count(&self, filter: Option<CurrencyFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);
        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: CurrencyFilter,
    ) -> Result<Vec<Currency>, RepositoryError> {
        self.query(Some(filter), None)
    }

    pub fn query(
        &self,
        filter: Option<CurrencyFilter>,
        sort: Option<CurrencySort>,
    ) -> Result<Vec<Currency>, RepositoryError> {
        let mut query = create_filtered_query(filter);
        if let Some(sort) = sort {
            match sort.key {
                CurrencySortField::Id => {
                    apply_sort_no_case!(query, sort, currency_dsl::id)
                }
                CurrencySortField::CurrencyCode => {
                    apply_sort_no_case!(query, sort, currency_dsl::code)
                }
            }
        } else {
            query = query.order(currency_dsl::code.asc())
        }

        let result = query.load::<CurrencyRow>(&self.connection.connection)?;

        Ok(result.into_iter().map(to_domain).collect())
    }
}

type BoxedLogQuery = currency::BoxedQuery<'static, DBType>;

fn create_filtered_query(filter: Option<CurrencyFilter>) -> BoxedLogQuery {
    let mut query = currency::table.into_boxed();

    if let Some(filter) = filter {
        apply_equal_filter!(query, filter.id, currency_dsl::id);

        query = match filter.is_home_currency {
            Some(true) => query.filter(currency_dsl::is_home_currency.eq(true)),
            Some(false) => query.filter(currency_dsl::is_home_currency.eq(false)),
            None => query,
        };

        query = match filter.is_active {
            Some(true) => query.filter(currency_dsl::is_active.eq(true)),
            Some(false) => query.filter(currency_dsl::is_active.eq(false)),
            None => query,
        };
    }
    query
}

pub fn to_domain(currency_row: CurrencyRow) -> Currency {
    Currency { currency_row }
}

impl CurrencyFilter {
    pub fn new() -> CurrencyFilter {
        CurrencyFilter {
            id: None,
            is_home_currency: None,
            is_active: None,
        }
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn is_home_currency(mut self, filter: bool) -> Self {
        self.is_home_currency = Some(filter);
        self
    }

    pub fn is_active(mut self, filter: bool) -> Self {
        self.is_active = Some(filter);
        self
    }
}

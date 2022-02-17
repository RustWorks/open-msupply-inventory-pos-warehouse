use crate::schema::queries::{ItemSortFieldInput, NameSortFieldInput, StocktakeSortFieldInput};

use super::{
    InvoiceNodeStatus, InvoiceNodeType, InvoiceSortFieldInput, LocationSortFieldInput,
    RequisitionNodeStatus, RequisitionNodeType, StocktakeNodeStatus,
};

use domain::{
    invoice::{InvoiceStatus, InvoiceType},
    DatetimeFilter, EqualFilter, SimpleStringFilter, Sort,
};

use async_graphql::{InputObject, InputType};
use chrono::{DateTime, Utc};

#[derive(InputObject)]
#[graphql(concrete(name = "InvoiceSortInput", params(InvoiceSortFieldInput)))]
#[graphql(concrete(name = "ItemSortInput", params(ItemSortFieldInput)))]
#[graphql(concrete(name = "NameSortInput", params(NameSortFieldInput)))]
#[graphql(concrete(name = "LocationSortInput", params(LocationSortFieldInput)))]
#[graphql(concrete(name = "StocktakeSortInput", params(StocktakeSortFieldInput)))]
pub struct SortInput<T: InputType> {
    /// Sort query result by `key`
    pub key: T,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    pub desc: Option<bool>,
}

impl<TInput, T> From<SortInput<TInput>> for Sort<T>
where
    TInput: InputType,
    T: From<TInput>,
{
    fn from(sort: SortInput<TInput>) -> Self {
        Sort {
            key: T::from(sort.key),
            desc: sort.desc,
        }
    }
}

pub fn convert_sort<FromField, ToField>(
    from: Option<Vec<SortInput<FromField>>>,
) -> Option<Sort<ToField>>
where
    FromField: InputType,
    Sort<ToField>: From<SortInput<FromField>>,
{
    // Currently only one sort option is supported, use the first from the list.
    from.map(|mut sort_list| sort_list.pop())
        .flatten()
        .map(Sort::from)
}

// simple string filter

#[derive(InputObject, Clone)]
pub struct SimpleStringFilterInput {
    /// Search term must be an exact match (case sensitive)
    equal_to: Option<String>,
    /// Search term must be included in search candidate (case insensitive)
    like: Option<String>,
}

impl From<SimpleStringFilterInput> for SimpleStringFilter {
    fn from(f: SimpleStringFilterInput) -> Self {
        SimpleStringFilter {
            equal_to: f.equal_to,
            like: f.like,
        }
    }
}

#[derive(InputObject, Clone)]
#[graphql(concrete(name = "EqualFilterStringInput", params(String)))]
#[graphql(concrete(name = "EqualFilterBooleanInput", params(bool)))]
#[graphql(concrete(name = "EqualFilterNumberInput", params(i32)))]
#[graphql(concrete(name = "EqualFilterBigNumberInput", params(i64)))]
#[graphql(concrete(name = "EqualFilterInvoiceTypeInput", params(InvoiceNodeType)))]
#[graphql(concrete(name = "EqualFilterInvoiceStatusInput", params(InvoiceNodeStatus)))]
#[graphql(concrete(
    name = "EqualFilterRequisitionStatusInput",
    params(RequisitionNodeStatus)
))]
#[graphql(concrete(name = "EqualFilterRequisitionTypeInput", params(RequisitionNodeType)))]
#[graphql(concrete(name = "EqualFilterStocktakeStatusInput", params(StocktakeNodeStatus)))]
pub struct EqualFilterInput<T: InputType> {
    pub equal_to: Option<T>,
    pub equal_any: Option<Vec<T>>,
    pub not_equal_to: Option<T>,
}

pub type EqualFilterBoolInput = EqualFilterInput<bool>;
pub type EqualFilterStringInput = EqualFilterInput<String>;
pub type EqualFilterNumberInput = EqualFilterInput<i32>;
pub type EqualFilterBigNumberInput = EqualFilterInput<i64>;

impl<I: InputType> EqualFilterInput<I> {
    pub fn map_to_domain<F, T>(self, to_domain: F) -> EqualFilter<T>
    where
        F: Fn(I) -> T,
    {
        EqualFilter {
            equal_to: self.equal_to.map(&to_domain),
            not_equal_to: self.not_equal_to.map(&to_domain),
            equal_any: self
                .equal_any
                .map(|inputs| inputs.into_iter().map(&to_domain).collect()),
            not_equal_all: None,
        }
    }
}

impl<T> From<EqualFilterInput<T>> for EqualFilter<T>
where
    T: InputType,
{
    fn from(
        EqualFilterInput {
            equal_to,
            equal_any,
            not_equal_to,
        }: EqualFilterInput<T>,
    ) -> Self {
        EqualFilter {
            equal_to,
            equal_any,
            not_equal_to,
            not_equal_all: None,
        }
    }
}

impl From<EqualFilterInput<InvoiceNodeType>> for EqualFilter<InvoiceType> {
    fn from(
        EqualFilterInput {
            equal_to,
            equal_any,
            not_equal_to,
        }: EqualFilterInput<InvoiceNodeType>,
    ) -> Self {
        EqualFilter {
            equal_to: equal_to.map(InvoiceType::from),
            equal_any: equal_any.map(|types| types.into_iter().map(InvoiceType::from).collect()),
            not_equal_to: not_equal_to.map(InvoiceType::from),
            not_equal_all: None,
        }
    }
}

impl From<EqualFilterInput<InvoiceNodeStatus>> for EqualFilter<InvoiceStatus> {
    fn from(
        EqualFilterInput {
            equal_to,
            equal_any,
            not_equal_to,
        }: EqualFilterInput<InvoiceNodeStatus>,
    ) -> Self {
        EqualFilter {
            equal_to: equal_to.map(InvoiceStatus::from),
            equal_any: equal_any.map(|types| types.into_iter().map(InvoiceStatus::from).collect()),
            not_equal_to: not_equal_to.map(InvoiceStatus::from),
            not_equal_all: None,
        }
    }
}

// Datetime filter

#[derive(InputObject, Clone)]
pub struct DatetimeFilterInput {
    pub equal_to: Option<DateTime<Utc>>,
    pub before_or_equal_to: Option<DateTime<Utc>>,
    pub after_or_equal_to: Option<DateTime<Utc>>,
}

impl From<DatetimeFilterInput> for DatetimeFilter {
    fn from(f: DatetimeFilterInput) -> Self {
        DatetimeFilter {
            equal_to: f.equal_to.map(|t| t.naive_utc()),
            before_or_equal_to: f.before_or_equal_to.map(|t| t.naive_utc()),
            after_or_equal_to: f.after_or_equal_to.map(|t| t.naive_utc()),
        }
    }
}

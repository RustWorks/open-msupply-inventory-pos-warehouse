pub mod insert;
use async_graphql::Object;
use graphql_types::types::StockLineConnector;
pub use insert::*;

pub mod delete;
pub use delete::*;

pub mod update;
pub use update::*;
pub struct StockLineReducedBelowZero(pub StockLineConnector);

#[Object]
impl StockLineReducedBelowZero {
    pub async fn description(&self) -> &'static str {
        "Stock line exist in new outbound shipments. "
    }

    pub async fn stock_line(&self) -> &StockLineConnector {
        &self.0
    }
}
pub struct AdjustmentReasonNotProvided;

#[Object]
impl AdjustmentReasonNotProvided {
    pub async fn description(&self) -> &'static str {
        "Stocktake line has no adjustment reason"
    }
}

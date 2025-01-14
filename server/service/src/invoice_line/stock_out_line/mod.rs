use repository::InvoiceType;

pub mod insert;
pub use self::insert::*;

pub mod update;
pub use self::update::*;

pub mod delete;
pub use self::delete::*;

pub mod validate;
pub use self::validate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum StockOutType {
    OutboundShipment,
    OutboundReturn,
    Prescription,
}

impl StockOutType {
    pub fn to_domain(&self) -> InvoiceType {
        match self {
            StockOutType::OutboundShipment => InvoiceType::OutboundShipment,
            StockOutType::Prescription => InvoiceType::Prescription,
            StockOutType::OutboundReturn => InvoiceType::OutboundReturn,
        }
    }
}

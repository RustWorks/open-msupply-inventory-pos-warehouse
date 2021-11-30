use crate::WithDBError;
use domain::outbound_shipment::DeleteOutboundShipmentLine;
use repository::{
    schema::InvoiceRowStatus, InvoiceLineRowRepository, InvoiceRepository, RepositoryError,
    StockLineRowRepository, StorageConnectionManager, TransactionError,
};

mod validate;

use validate::validate;

pub fn delete_outbound_shipment_service_line(
    connection_manager: &StorageConnectionManager,
    input: DeleteOutboundShipmentLine,
) -> Result<String, DeleteOutboundShipmentServiceLineError> {
    let connection = connection_manager.connection()?;

    let line = connection
        .transaction_sync(|connection| {
            let line = validate(&input, &connection)?;
            let stock_line_id_option = line.stock_line_id.clone();

            InvoiceLineRowRepository::new(&connection).delete(&line.id)?;

            if let Some(stock_line_id) = stock_line_id_option {
                let invoice_repository = InvoiceRepository::new(&connection);
                let stock_line_repository = StockLineRowRepository::new(&connection);

                let mut stock_line = stock_line_repository.find_one_by_id(&stock_line_id)?;
                stock_line.available_number_of_packs += line.number_of_packs;

                let invoice = invoice_repository.find_one_by_id(&line.invoice_id)?;
                if invoice.status == InvoiceRowStatus::Confirmed {
                    stock_line.total_number_of_packs += line.number_of_packs;
                }

                stock_line_repository.upsert_one(&stock_line)?;
            }
            Ok(line)
        })
        .map_err(
            |error: TransactionError<DeleteOutboundShipmentServiceLineError>| match error {
                TransactionError::Transaction { msg, level } => {
                    RepositoryError::TransactionError { msg, level }.into()
                }
                TransactionError::Inner(error) => error,
            },
        )?;
    Ok(line.id)
}

pub enum DeleteOutboundShipmentServiceLineError {
    LineDoesNotExist,
    DatabaseError(RepositoryError),
    InvoiceDoesNotExist,
    NotAnOutboundShipment,
    NotThisStoreInvoice,
    ItemNotFound,
    CannotEditFinalised,
    NotThisInvoiceLine(String),
    NotAServiceItem,
}

impl From<RepositoryError> for DeleteOutboundShipmentServiceLineError {
    fn from(error: RepositoryError) -> Self {
        DeleteOutboundShipmentServiceLineError::DatabaseError(error)
    }
}

impl<ERR> From<WithDBError<ERR>> for DeleteOutboundShipmentServiceLineError
where
    ERR: Into<DeleteOutboundShipmentServiceLineError>,
{
    fn from(result: WithDBError<ERR>) -> Self {
        match result {
            WithDBError::DatabaseError(error) => error.into(),
            WithDBError::Error(error) => error.into(),
        }
    }
}

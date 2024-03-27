use repository::{Invoice, InvoiceRow, InvoiceRowStatus, InvoiceRowType, Name};

use crate::invoice::{
    check_invoice_does_not_exists, check_invoice_type, check_store, get_invoice,
    InvoiceAlreadyExistsError,
};
use crate::service_provider::ServiceContext;
use crate::validate::{check_other_party, get_other_party, CheckOtherPartyType, OtherPartyErrors};

use super::{InsertInboundReturn, InsertInboundReturnError, ShipmentOrNameId};

pub fn validate(
    ctx: &ServiceContext,
    store_id: &str,
    input: &InsertInboundReturn,
) -> Result<Name, InsertInboundReturnError> {
    use InsertInboundReturnError::*;

    let ServiceContext { connection, .. } = ctx;

    check_invoice_does_not_exists(&input.id, connection).map_err(|e| match e {
        InvoiceAlreadyExistsError::InvoiceAlreadyExists => InvoiceAlreadyExists,
        InvoiceAlreadyExistsError::RepositoryError(err) => DatabaseError(err),
    })?;

    let other_party = match &input.shipment_or_name_id {
        ShipmentOrNameId::ShipmentId(shipment_id) => {
            let Invoice {
                invoice_row: outbound_shipment,
                name_row,
                ..
            } = get_invoice(ctx, None, &shipment_id)?.ok_or(OutboundShipmentDoesNotExist)?;

            if !check_store(&outbound_shipment, store_id) {
                return Err(OutboundShipmentDoesNotBelongToCurrentStore);
            }
            if !check_invoice_type(&outbound_shipment, InvoiceRowType::OutboundShipment) {
                return Err(OriginalInvoiceNotAnOutboundShipment);
            }
            if !check_outbound_shipment_is_returnable(&outbound_shipment) {
                return Err(CannotReturnOutboundShipment);
            }

            get_other_party(connection, store_id, &name_row.id)?.ok_or(OtherPartyDoesNotExist {})?
        }
        ShipmentOrNameId::NameId(other_party_id) => check_other_party(
            connection,
            store_id,
            &other_party_id,
            CheckOtherPartyType::Customer,
        )
        .map_err(|e| match e {
            OtherPartyErrors::OtherPartyDoesNotExist => OtherPartyDoesNotExist {},
            OtherPartyErrors::OtherPartyNotVisible => OtherPartyNotVisible,
            OtherPartyErrors::TypeMismatched => OtherPartyNotACustomer,
            OtherPartyErrors::DatabaseError(repository_error) => DatabaseError(repository_error),
        })?,
    };

    Ok(other_party)
}

fn check_outbound_shipment_is_returnable(outbound_shipment: &InvoiceRow) -> bool {
    match outbound_shipment.status {
        InvoiceRowStatus::Shipped | InvoiceRowStatus::Delivered | InvoiceRowStatus::Verified => {
            true
        }
        _ => false,
    }
}

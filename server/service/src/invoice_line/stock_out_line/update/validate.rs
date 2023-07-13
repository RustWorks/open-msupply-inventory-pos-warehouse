use repository::{InvoiceLineRow, InvoiceRow, ItemRow, StorageConnection};

use crate::{
    invoice::{check_invoice_exists, check_invoice_is_editable, check_invoice_type, check_store},
    invoice_line::{
        outbound_shipment_line::{
            check_batch_exists, check_batch_on_hold, check_item_matches_batch,
            check_location_on_hold, check_unique_stock_line, LocationIsOnHoldError,
        },
        stock_out_line::BatchPair,
        validate::{
            check_item_exists, check_line_belongs_to_invoice, check_line_exists_option,
            check_number_of_packs,
        },
    },
};

use super::{UpdateStockOutLine, UpdateStockOutLineError};

pub fn validate(
    input: &UpdateStockOutLine,
    store_id: &str,
    connection: &StorageConnection,
) -> Result<(InvoiceLineRow, ItemRow, BatchPair, InvoiceRow), UpdateStockOutLineError> {
    use UpdateStockOutLineError::*;

    let line = check_line_exists_option(connection, &input.id)?.ok_or(LineDoesNotExist)?;
    let invoice = check_invoice_exists(&line.invoice_id, connection)?.ok_or(InvoiceDoesNotExist)?;
    if !check_store(&invoice, store_id) {
        return Err(NotThisStoreInvoice);
    }
    let unique_stock = check_unique_stock_line(
        &line.id.clone(),
        &invoice.id,
        input.stock_line_id.clone(),
        connection,
    )?;
    if unique_stock.is_some() {
        return Err(StockLineAlreadyExistsInInvoice(unique_stock.unwrap().id));
    }

    if let Some(r#type) = &input.r#type {
        if !check_invoice_type(&invoice, r#type.to_domain()) {
            return Err(InvoiceTypeDoesNotMatch);
        }
    } else {
        return Err(NoInvoiceType);
    }
    if !check_invoice_is_editable(&invoice) {
        return Err(CannotEditFinalised);
    }
    if !check_line_belongs_to_invoice(&line, &invoice) {
        return Err(NotThisInvoiceLine(line.invoice_id));
    }
    if !check_number_of_packs(input.number_of_packs.clone()) {
        return Err(NumberOfPacksBelowOne);
    }

    let batch_pair = check_batch_exists_option(&input, &line, connection)?;
    let item = check_item_option(input.item_id.clone(), &line, connection)?;

    if !check_item_matches_batch(&batch_pair.main_batch, &item) {
        return Err(ItemDoesNotMatchStockLine);
    }
    if !check_batch_on_hold(&batch_pair.main_batch) {
        return Err(BatchIsOnHold);
    }
    check_location_on_hold(&batch_pair.main_batch, connection).map_err(|e| match e {
        LocationIsOnHoldError::LocationIsOnHold => LocationIsOnHold,
        LocationIsOnHoldError::LocationNotFound => LocationNotFound,
    })?;
    check_reduction_below_zero(&input, &line, &batch_pair)?;

    Ok((line, item, batch_pair, invoice))
}

fn check_reduction_below_zero(
    input: &UpdateStockOutLine,
    line: &InvoiceLineRow,
    batch_pair: &BatchPair,
) -> Result<(), UpdateStockOutLineError> {
    // If previous batch is present, this means we are adjust new batch thus:
    // - check full number of pack in invoice
    let reduction = batch_pair.get_main_batch_reduction(input, line);

    if batch_pair.main_batch.available_number_of_packs < reduction {
        Err(UpdateStockOutLineError::ReductionBelowZero {
            stock_line_id: batch_pair.main_batch.id.clone(),
            line_id: line.id.clone(),
        })
    } else {
        Ok(())
    }
}

fn check_item_option(
    item_id: Option<String>,
    invoice_line: &InvoiceLineRow,
    connection: &StorageConnection,
) -> Result<ItemRow, UpdateStockOutLineError> {
    if let Some(item_id) = item_id {
        Ok(
            check_item_exists(connection, &item_id)?
                .ok_or(UpdateStockOutLineError::ItemNotFound)?,
        )
    } else {
        Ok(check_item_exists(connection, &invoice_line.item_id)?
            .ok_or(UpdateStockOutLineError::ItemNotFound)?)
    }
}

fn check_batch_exists_option(
    input: &UpdateStockOutLine,
    existing_line: &InvoiceLineRow,
    connection: &StorageConnection,
) -> Result<BatchPair, UpdateStockOutLineError> {
    use UpdateStockOutLineError::*;

    let previous_batch = if let Some(batch_id) = &existing_line.stock_line_id {
        // Should always be found due to contraints on database
        check_batch_exists(batch_id, connection)?.ok_or(StockLineNotFound)?
    } else {
        // This should never happen, but still need to cover
        return Err(LineDoesNotReferenceStockLine);
    };

    let result = match &input.stock_line_id {
        Some(batch_id) if batch_id != &previous_batch.id => BatchPair {
            // stock_line changed
            main_batch: check_batch_exists(batch_id, connection)?.ok_or(StockLineNotFound)?,
            previous_batch_option: Some(previous_batch),
        },
        _ => {
            // stock_line_id not changed
            BatchPair {
                main_batch: previous_batch,
                previous_batch_option: None,
            }
        }
    };

    Ok(result)
}

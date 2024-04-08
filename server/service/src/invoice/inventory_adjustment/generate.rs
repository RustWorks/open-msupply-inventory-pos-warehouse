use chrono::Utc;

use repository::{
    InvoiceRow, InvoiceRowStatus, InvoiceRowType, NumberRowType, RepositoryError, StorageConnection,
};
use repository::{NameRowRepository, StockLine, StockLineRow};
use util::constants::INVENTORY_ADJUSTMENT_NAME_CODE;
use util::uuid::uuid;

use crate::invoice_line::stock_in_line::{InsertStockInLine, StockInType};
use crate::invoice_line::stock_out_line::{InsertStockOutLine, StockOutType};
use crate::number::next_number;
use crate::{i32_to_u32, NullableUpdate};

use super::{AdjustmentType, InsertInventoryAdjustment};

pub fn generate(
    connection: &StorageConnection,
    store_id: &str,
    user_id: &str,
    InsertInventoryAdjustment {
        stock_line_id,
        adjustment,
        adjustment_type,
        inventory_adjustment_reason_id,
    }: InsertInventoryAdjustment,
    stock_line: StockLine,
) -> Result<
    (
        InvoiceRow,
        Option<InsertStockInLine>,
        Option<InsertStockOutLine>,
    ),
    RepositoryError,
> {
    let current_datetime = Utc::now().naive_utc();

    let inventory_adjustment_name = NameRowRepository::new(connection)
        .find_one_by_code(INVENTORY_ADJUSTMENT_NAME_CODE)?
        .ok_or(RepositoryError::NotFound)?;

    let invoice_number = next_number(
        connection,
        &match adjustment_type {
            AdjustmentType::Addition => NumberRowType::InventoryAddition,
            AdjustmentType::Reduction => NumberRowType::InventoryReduction,
        },
        store_id,
    )?;

    let invoice = InvoiceRow {
        id: uuid(),
        user_id: Some(user_id.to_string()),
        name_link_id: inventory_adjustment_name.id,
        r#type: InvoiceRowType::OutboundReturn,
        invoice_number,
        store_id: store_id.to_string(),
        created_datetime: current_datetime,
        status: InvoiceRowStatus::New,
        original_shipment_id: None,
        // Default
        currency_id: None,
        currency_rate: 1.0,
        on_hold: false,
        colour: None,
        comment: None,
        their_reference: None,
        tax: None,
        name_store_id: None,
        transport_reference: None,
        allocated_datetime: None,
        picked_datetime: None,
        shipped_datetime: None,
        delivered_datetime: None,
        verified_datetime: None,
        linked_invoice_id: None,
        requisition_id: None,
        clinician_link_id: None,
    };

    let StockLineRow {
        location_id,
        batch,
        expiry_date,
        pack_size,
        cost_price_per_pack,
        sell_price_per_pack,
        note,
        ..
    } = stock_line.stock_line_row;

    let line_id = uuid();

    let insert_stock_in_line: Option<InsertStockInLine>;
    let insert_stock_out_line: Option<InsertStockOutLine>;

    match adjustment_type {
        AdjustmentType::Addition => {
            let line = InsertStockInLine {
                id: line_id,
                invoice_id: invoice.id.clone(),
                item_id: stock_line.item_row.id,
                location: location_id.map(|id| NullableUpdate { value: Some(id) }),
                pack_size: i32_to_u32(pack_size),
                batch,
                note,
                cost_price_per_pack,
                sell_price_per_pack,
                expiry_date,
                number_of_packs: adjustment,
                stock_line_id: Some(stock_line_id),
                inventory_adjustment_reason_id,
                r#type: StockInType::InventoryAddition,
                total_before_tax: None,
                tax: None,
            };
            insert_stock_in_line = Some(line);
            insert_stock_out_line = None;
        }
        AdjustmentType::Reduction => {
            let line = InsertStockOutLine {
                id: line_id,
                invoice_id: invoice.id.clone(),
                stock_line_id,
                inventory_adjustment_reason_id,
                note,
                number_of_packs: adjustment,
                r#type: Some(StockOutType::InventoryReduction),
                total_before_tax: None,
                tax: None,
            };
            insert_stock_in_line = None;
            insert_stock_out_line = Some(line);
        }
    }

    Ok((invoice, insert_stock_in_line, insert_stock_out_line))
}

use crate::{
    invoice::common::{calculate_total_after_tax, generate_invoice_user_id_update},
    invoice_line::{
        convert_invoice_line_to_single_pack, convert_stock_line_to_single_pack, generate_batch,
        stock_in_line::StockInType,
    },
    store_preference::get_store_preferences,
    u32_to_i32,
};
use repository::{
    InvoiceLineRow, InvoiceLineRowType, InvoiceRow, InvoiceRowStatus, ItemRow, RepositoryError,
    StockLineRow, StorageConnection,
};

use super::InsertStockInLine;

pub fn generate(
    connection: &StorageConnection,
    user_id: &str,
    input: InsertStockInLine,
    item_row: ItemRow,
    existing_invoice_row: InvoiceRow,
) -> Result<(Option<InvoiceRow>, InvoiceLineRow, Option<StockLineRow>), RepositoryError> {
    let store_preferences = get_store_preferences(connection, &existing_invoice_row.store_id)?;

    let stock_in_type = input.r#type.clone();

    let new_line = generate_line(input, item_row, existing_invoice_row.clone());

    let mut new_line = match store_preferences.pack_to_one {
        true => convert_invoice_line_to_single_pack(new_line),
        false => new_line,
    };

    let new_batch_option = if should_upsert_batch(&stock_in_type, &existing_invoice_row) {
        let new_batch = generate_batch(
            &existing_invoice_row.store_id,
            new_line.clone(),
            should_keep_existing_batch(&stock_in_type),
            &existing_invoice_row.name_link_id,
        );
        new_line.stock_line_id = Some(new_batch.id.clone());

        let new_batch = match store_preferences.pack_to_one {
            true => convert_stock_line_to_single_pack(new_batch),
            false => new_batch,
        };

        Some(new_batch)
    } else {
        None
    };

    Ok((
        generate_invoice_user_id_update(user_id, existing_invoice_row),
        new_line,
        new_batch_option,
    ))
}

fn generate_line(
    InsertStockInLine {
        id,
        invoice_id,
        item_id,
        pack_size,
        batch,
        expiry_date,
        sell_price_per_pack,
        cost_price_per_pack,
        number_of_packs,
        location,
        total_before_tax,
        note,
        stock_line_id,
        inventory_adjustment_reason_id,
        tax: _,
        r#type: _,
    }: InsertStockInLine,
    ItemRow {
        name: item_name,
        code: item_code,
        ..
    }: ItemRow,
    InvoiceRow { tax, .. }: InvoiceRow,
) -> InvoiceLineRow {
    let total_before_tax = total_before_tax.unwrap_or(cost_price_per_pack * number_of_packs as f64);
    let total_after_tax = calculate_total_after_tax(total_before_tax, tax);
    InvoiceLineRow {
        id,
        invoice_id,
        item_link_id: item_id,
        location_id: location.map(|l| l.value).unwrap_or_default(),
        pack_size: u32_to_i32(pack_size),
        batch,
        expiry_date,
        sell_price_per_pack,
        cost_price_per_pack,
        r#type: InvoiceLineRowType::StockIn,
        number_of_packs,
        item_name,
        item_code,
        stock_line_id,
        total_before_tax,
        total_after_tax,
        tax,
        note,
        inventory_adjustment_reason_id,
        return_reason_id: None,
        foreign_currency_price_before_tax: None,
    }
}

fn should_upsert_batch(stock_in_type: &StockInType, existing_invoice_row: &InvoiceRow) -> bool {
    match stock_in_type {
        StockInType::InboundReturn => existing_invoice_row.status != InvoiceRowStatus::New,
        StockInType::InventoryAddition => true,
    }
}

fn should_keep_existing_batch(stock_in_type: &StockInType) -> bool {
    match stock_in_type {
        StockInType::InboundReturn => false,
        StockInType::InventoryAddition => true,
    }
}

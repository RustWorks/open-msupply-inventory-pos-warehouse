mod graphql {
    use crate::graphql::common::{
        assert_unwrap_enum, assert_unwrap_optional_key, compare_option,
        convert_graphql_client_type, get_invoice_inline, get_invoice_lines_inline,
    };
    use crate::graphql::get_gql_result;
    use crate::graphql::{
        invoice_full as get, update_supplier_invoice_line_full as update, InvoiceFull as Get,
        UpdateSupplierInvoiceLineFull as Update,
    };
    use chrono::NaiveDate;
    use graphql_client::{GraphQLQuery, Response};
    use remote_server::{
        database::{
            mock::MockDataInserts,
            repository::{InvoiceLineRepository, StockLineRepository},
            schema::{InvoiceLineRow, StockLineRow},
        },
        domain::{invoice::InvoiceFilter, Pagination},
        util::test_db,
    };

    use update::UpdateSupplierInvoiceLineErrorInterface::*;

    macro_rules! assert_unwrap_response_variant {
        ($response:ident) => {
            assert_unwrap_optional_key!($response, data).update_supplier_invoice_line
        };
    }

    macro_rules! assert_unwrap_line {
        ($response:ident) => {{
            let response_variant = assert_unwrap_response_variant!($response);
            assert_unwrap_enum!(
                response_variant,
                update::UpdateSupplierInvoiceLineResponse::InvoiceLineNode
            )
        }};
    }

    macro_rules! assert_unwrap_batch {
        ($line:ident) => {{
            let line_cloned = $line.clone();
            let batch_variant = assert_unwrap_optional_key!(line_cloned, stock_line);
            let batch =
                assert_unwrap_enum!(batch_variant, update::StockLineResponse::StockLineNode);
            batch
        }};
    }

    macro_rules! assert_unwrap_error {
        ($response:ident) => {{
            let response_variant = assert_unwrap_response_variant!($response);
            let error_wrapper = assert_unwrap_enum!(
                response_variant,
                update::UpdateSupplierInvoiceLineResponse::UpdateSupplierInvoiceLineError
            );
            error_wrapper.error
        }};
    }

    macro_rules! assert_error {
        ($response:ident, $error:expr) => {{
            let lhs = assert_unwrap_error!($response);
            let rhs = $error;
            assert_eq!(lhs, rhs);
        }};
    }

    #[actix_rt::test]
    async fn test_update_supplier_invoice_line() {
        let (mock_data, connection, settings) = test_db::setup_all(
            "test_update_supplier_invoice_line_query",
            MockDataInserts::all(),
        )
        .await;

        // Setup

        let draft_supplier_invoice = get_invoice_inline!(
            InvoiceFilter::new()
                .match_supplier_invoice()
                .match_draft()
                .match_id("supplier_invoice_c"),
            &connection
        );
        let confirmed_supplier_invoice = get_invoice_inline!(
            InvoiceFilter::new()
                .match_supplier_invoice()
                .match_confirmed()
                .match_id("supplier_invoice_d"),
            &connection
        );
        let finalised_supplier_invoice = get_invoice_inline!(
            InvoiceFilter::new()
                .match_supplier_invoice()
                .match_finalised(),
            &connection
        );
        let customer_invoice =
            get_invoice_inline!(InvoiceFilter::new().match_customer_invoice(), &connection);
        let item = mock_data.items.first().unwrap();
        let confirmed_invoice_lines =
            get_invoice_lines_inline!(&confirmed_supplier_invoice.id.clone(), &connection);
        let customer_invoice_lines =
            get_invoice_lines_inline!(&customer_invoice.id.clone(), &connection);
        let finalised_invoice_lines =
            get_invoice_lines_inline!(&finalised_supplier_invoice.id.clone(), &connection);
        let draft_invoice_lines =
            get_invoice_lines_inline!(&draft_supplier_invoice.id.clone(), &connection);

        let base_variables = update::Variables {
            id: draft_invoice_lines[0].id.clone(),
            invoice_id_usil: draft_supplier_invoice.id.clone(),
            item_id_usil: Some(item.id.clone()),
            cost_price_per_pack_usil: Some(5.5),
            sell_price_per_pack_usil: Some(7.7),
            pack_size_usil: Some(3),
            number_of_packs_usil: Some(9),
            expiry_date_usil: Some(NaiveDate::from_ymd(2020, 8, 3)),
            batch_usil: Some("some batch name".to_string()),
        };

        // Test RecordDoesNotExist Item

        let mut variables = base_variables.clone();
        variables.id = "invalid".to_string();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;

        assert_error!(
            response,
            RecordDoesNotExist(update::RecordDoesNotExist {
                description: "Record does not exist".to_string(),
            })
        );

        // Test ForeingKeyError Item

        let mut variables = base_variables.clone();
        variables.item_id_usil = Some("invalid".to_string());

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            ForeignKeyError(update::ForeignKeyError {
                description: "FK record doesn't exist".to_string(),
                key: update::ForeignKey::ItemId,
            })
        );

        // Test ForeingKeyError Invoice

        let mut variables = base_variables.clone();
        variables.invoice_id_usil = "invalid".to_string();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            ForeignKeyError(update::ForeignKeyError {
                description: "FK record doesn't exist".to_string(),
                key: update::ForeignKey::InvoiceId,
            })
        );

        // Test CannotEditFinalisedInvoice

        let mut variables = base_variables.clone();
        variables.id = finalised_invoice_lines[0].id.clone();
        variables.invoice_id_usil = finalised_supplier_invoice.id.clone();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            CannotEditFinalisedInvoice(update::CannotEditFinalisedInvoice {
                description: "Cannot edit finalised invoice".to_string(),
            },)
        );

        // Test NotASupplierInvoice

        let mut variables = base_variables.clone();
        variables.id = customer_invoice_lines[0].id.clone();
        variables.invoice_id_usil = customer_invoice.id.clone();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            NotASupplierInvoice(update::NotASupplierInvoice {
                description: "Invoice is not Supplier Invoice".to_string(),
            })
        );

        // Test RangeError NumberOfPacks

        let mut variables = base_variables.clone();
        variables.number_of_packs_usil = Some(0);

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            RangeError(update::RangeError {
                description: "Value is below minimum".to_string(),
                field: update::RangeField::NumberOfPacks,
                max: None,
                min: Some(1),
            })
        );

        // Test RangeError PackSize

        let mut variables = base_variables.clone();
        variables.number_of_packs_usil = Some(0);

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            RangeError(update::RangeError {
                description: "Value is below minimum".to_string(),
                field: update::RangeField::NumberOfPacks,
                max: None,
                min: Some(1),
            })
        );

        // Test InvoiceLineBelongsToAnotherInvoice

        let mut variables = base_variables.clone();
        variables.invoice_id_usil = confirmed_supplier_invoice.id.clone();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        let invoice: Response<get::ResponseData> = get_gql_result(
            &settings,
            Get::build_query(get::Variables {
                id: draft_supplier_invoice.id,
            }),
        )
        .await;

        assert_error!(
            response,
            InvoiceLineBelongsToAnotherInvoice(update::InvoiceLineBelongsToAnotherInvoice {
                description: "Invoice line belongs to another invoice".to_string(),
                invoice: convert_graphql_client_type(invoice.data.unwrap().invoice)
            },)
        );

        // Test BatchIsReserved

        let mut variables = base_variables.clone();
        variables.id = confirmed_invoice_lines[1].id.clone();
        variables.invoice_id_usil = confirmed_supplier_invoice.id.clone();
        let mut stock_line = StockLineRepository::new(&connection)
            .find_one_by_id(&confirmed_invoice_lines[1].stock_line_id.clone().unwrap())
            .unwrap();
        stock_line.available_number_of_packs -= 1;
        StockLineRepository::new(&connection)
            .upsert_one(&stock_line)
            .unwrap();

        let query = Update::build_query(variables);
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;

        assert_error!(
            response,
            BatchIsReserved(update::BatchIsReserved {
                description: "Batch is already reserved/issued".to_string(),
            })
        );

        // Success Draft

        let variables = base_variables.clone();

        let query = Update::build_query(variables.clone());
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);
        assert_eq!(line.id, variables.id);
        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&variables.id)
            .unwrap();
        assert_eq!(new_line, variables);
        assert_eq!(new_line.stock_line_id, None);
        assert_eq!(
            new_line.total_after_tax,
            new_line.pack_size as f64
                * new_line.number_of_packs as f64
                * new_line.cost_price_per_pack
        );

        // Success Confirmed

        let mut variables = base_variables.clone();
        variables.id = confirmed_invoice_lines[0].id.clone();
        variables.invoice_id_usil = confirmed_supplier_invoice.id.clone();

        let query = Update::build_query(variables.clone());
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);
        let batch = assert_unwrap_batch!(line);

        assert_eq!(line.id, variables.id);

        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&variables.id)
            .unwrap();
        let new_stock_line = StockLineRepository::new(&connection)
            .find_one_by_id(&batch.id)
            .unwrap();

        assert_eq!(new_line, variables);
        assert_eq!(new_stock_line, variables);
        assert_eq!(new_line.stock_line_id, Some(new_stock_line.id));

        assert_eq!(
            new_line.total_after_tax,
            new_line.pack_size as f64
                * new_line.number_of_packs as f64
                * new_line.cost_price_per_pack
        );

        // Success Confirmed make batch name and expiry null

        // Need nullable and option input

        // Success Confirmed Nothing Changed

        let variables = update::Variables {
            id: confirmed_invoice_lines[0].id.clone(),
            invoice_id_usil: confirmed_supplier_invoice.id.clone(),
            item_id_usil: None,
            cost_price_per_pack_usil: None,
            sell_price_per_pack_usil: None,
            pack_size_usil: None,
            number_of_packs_usil: None,
            expiry_date_usil: None,
            batch_usil: None,
        };
        let start_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&variables.id)
            .unwrap();
        let start_batch = StockLineRepository::new(&connection)
            .find_one_by_id(&batch.id)
            .unwrap();

        let query = Update::build_query(variables.clone());
        let response: Response<update::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);
        let batch = assert_unwrap_batch!(line);

        assert_eq!(line.id, variables.id);

        let end_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&variables.id)
            .unwrap();
        let end_batch = StockLineRepository::new(&connection)
            .find_one_by_id(&batch.id)
            .unwrap();

        assert_eq!(start_line, end_line);
        assert_eq!(start_batch, end_batch);
    }

    impl PartialEq<update::Variables> for InvoiceLineRow {
        fn eq(&self, other: &update::Variables) -> bool {
            let update::Variables {
                batch_usil,
                cost_price_per_pack_usil,
                expiry_date_usil,
                id: id_usil,
                invoice_id_usil,
                item_id_usil,
                number_of_packs_usil,
                sell_price_per_pack_usil,
                pack_size_usil,
            } = other;

            compare_option(cost_price_per_pack_usil, &self.cost_price_per_pack)
                && *expiry_date_usil == self.expiry_date
                && *id_usil == self.id
                && *invoice_id_usil == self.invoice_id
                && compare_option(item_id_usil, &self.item_id)
                && compare_option(number_of_packs_usil, &(self.number_of_packs as i64))
                && compare_option(sell_price_per_pack_usil, &self.sell_price_per_pack)
                && *batch_usil == self.batch
                && compare_option(pack_size_usil, &(self.pack_size as i64))
        }
    }

    impl PartialEq<update::Variables> for StockLineRow {
        fn eq(&self, other: &update::Variables) -> bool {
            let update::Variables {
                batch_usil,
                cost_price_per_pack_usil,
                expiry_date_usil,
                id: _,
                invoice_id_usil: _,
                item_id_usil,
                number_of_packs_usil,
                sell_price_per_pack_usil,
                pack_size_usil,
            } = other;

            compare_option(cost_price_per_pack_usil, &self.cost_price_per_pack)
                && *expiry_date_usil == self.expiry_date
                && compare_option(item_id_usil, &self.item_id)
                && compare_option(
                    number_of_packs_usil,
                    &(self.available_number_of_packs as i64),
                )
                && compare_option(number_of_packs_usil, &(self.total_number_of_packs as i64))
                && compare_option(sell_price_per_pack_usil, &self.sell_price_per_pack)
                && *batch_usil == self.batch
                && compare_option(pack_size_usil, &(self.pack_size as i64))
        }
    }
}

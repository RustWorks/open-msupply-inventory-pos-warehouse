mod graphql {
    use crate::graphql::common::{
        assert_unwrap_enum, assert_unwrap_optional_key, get_invoice_inline,
    };
    use crate::graphql::{
        get_gql_result, insert_supplier_invoice_line_full as insert,
        InsertSupplierInvoiceLineFull as Insert,
    };
    use chrono::NaiveDate;
    use graphql_client::{GraphQLQuery, Response};
    use insert::InsertSupplierInvoiceLineErrorInterface::*;
    use remote_server::{
        database::{
            mock::MockDataInserts,
            repository::{InvoiceLineRepository, StockLineRepository},
            schema::{InvoiceLineRow, StockLineRow},
        },
        domain::{invoice::InvoiceFilter, Pagination},
        util::test_db,
    };
    use uuid::Uuid;

    macro_rules! assert_unwrap_response_variant {
        ($response:ident) => {
            assert_unwrap_optional_key!($response, data).insert_supplier_invoice_line
        };
    }

    macro_rules! assert_unwrap_line {
        ($response:ident) => {{
            let response_variant = assert_unwrap_response_variant!($response);
            assert_unwrap_enum!(
                response_variant,
                insert::InsertSupplierInvoiceLineResponse::InvoiceLineNode
            )
        }};
    }

    macro_rules! assert_unwrap_batch {
        ($line:ident) => {{
            let line_cloned = $line.clone();
            let batch_variant = assert_unwrap_optional_key!(line_cloned, stock_line);
            let batch =
                assert_unwrap_enum!(batch_variant, insert::StockLineResponse::StockLineNode);
            batch
        }};
    }

    macro_rules! assert_unwrap_error {
        ($response:ident) => {{
            let response_variant = assert_unwrap_response_variant!($response);
            let error_wrapper = assert_unwrap_enum!(
                response_variant,
                insert::InsertSupplierInvoiceLineResponse::InsertSupplierInvoiceLineError
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
    async fn test_insert_supplier_invoice_line() {
        let (mut mock_data, connection, settings) = test_db::setup_all(
            "test_insert_supplier_invoice_line_query",
            MockDataInserts::all(),
        )
        .await;

        // Setup

        let draft_supplier_invoice = get_invoice_inline!(
            InvoiceFilter::new().match_supplier_invoice().match_draft(),
            &connection
        );
        let confirmed_supplier_invoice = get_invoice_inline!(
            InvoiceFilter::new()
                .match_supplier_invoice()
                .match_confirmed(),
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
        let item = mock_data.items.pop().unwrap();
        let existing_line = mock_data.invoice_lines.pop().unwrap();

        let base_variables = insert::Variables {
            id: Uuid::new_v4().to_string(),
            invoice_id_isil: draft_supplier_invoice.id.clone(),
            item_id_isil: item.id.clone(),
            cost_price_per_pack_isil: 5.5,
            sell_price_per_pack_isil: 7.7,
            pack_size_isil: 3,
            number_of_packs_isil: 9,
            expiry_date_isil: Some(NaiveDate::from_ymd(2020, 8, 3)),
            batch_isil: Some("some batch name".to_string()),
        };

        // Test ForeingKeyError Item

        let mut variables = base_variables.clone();
        variables.item_id_isil = "invalid".to_string();

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            ForeignKeyError(insert::ForeignKeyError {
                description: "FK record doesn't exist".to_string(),
                key: insert::ForeignKey::ItemId,
            })
        );
        // Test ForeingKeyError Invoice

        let mut variables = base_variables.clone();
        variables.invoice_id_isil = "invalid".to_string();

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            ForeignKeyError(insert::ForeignKeyError {
                description: "FK record doesn't exist".to_string(),
                key: insert::ForeignKey::InvoiceId,
            })
        );
        // Test CannotEditFinalisedInvoice

        let mut variables = base_variables.clone();
        variables.invoice_id_isil = finalised_supplier_invoice.id.clone();

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            CannotEditFinalisedInvoice(insert::CannotEditFinalisedInvoice {
                description: "Cannot edit finalised invoice".to_string(),
            },)
        );

        // Test NotASupplierInvoice

        let mut variables = base_variables.clone();
        variables.invoice_id_isil = customer_invoice.id.clone();

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            NotASupplierInvoice(insert::NotASupplierInvoice {
                description: "Invoice is not Supplier Invoice".to_string(),
            })
        );
        // Test RangeError NumberOfPacks

        let mut variables = base_variables.clone();
        variables.number_of_packs_isil = 0;

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            RangeError(insert::RangeError {
                description: "Value is below minimum".to_string(),
                field: insert::RangeField::NumberOfPacks,
                max: None,
                min: Some(1),
            })
        );
        // Test RangeError PackSize

        let mut variables = base_variables.clone();
        variables.number_of_packs_isil = 0;

        let query = Insert::build_query(variables);
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            RangeError(insert::RangeError {
                description: "Value is below minimum".to_string(),
                field: insert::RangeField::NumberOfPacks,
                max: None,
                min: Some(1),
            })
        );
        // Test RecordAlreadyExists

        let mut variables = base_variables.clone();
        variables.id = existing_line.id.clone();

        let query = Insert::build_query(variables);

        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        assert_error!(
            response,
            RecordAlreadyExist(insert::RecordAlreadyExist {
                description: "Record already exists".to_string(),
            })
        );
        // Success Draft

        let variables = base_variables.clone();

        let query = Insert::build_query(variables.clone());
        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);

        assert_eq!(line.id, variables.id);

        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&variables.id)
            .unwrap();

        assert_eq!(new_line, variables);

        // Success Confirmed

        let mut variables = base_variables.clone();
        variables.id = Uuid::new_v4().to_string();
        variables.invoice_id_isil = confirmed_supplier_invoice.id.clone();

        let query = Insert::build_query(variables.clone());

        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);
        let batch = assert_unwrap_batch!(line);

        assert_eq!(line.id, variables.id);

        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&line.id)
            .unwrap();
        let new_stock_line = StockLineRepository::new(&connection)
            .find_one_by_id(&batch.id)
            .unwrap();

        assert_eq!(new_line, variables);
        assert_eq!(new_stock_line, variables);

        // Success Confirmed

        let mut variables = base_variables.clone();
        variables.id = Uuid::new_v4().to_string();
        variables.expiry_date_isil = None;
        variables.batch_isil = None;
        variables.invoice_id_isil = confirmed_supplier_invoice.id.clone();

        let query = Insert::build_query(variables.clone());

        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);
        let batch = assert_unwrap_batch!(line);

        assert_eq!(line.id, variables.id);

        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&line.id)
            .unwrap();
        let new_stock_line = StockLineRepository::new(&connection)
            .find_one_by_id(&batch.id)
            .unwrap();

        assert_eq!(new_line, variables);
        assert_eq!(new_stock_line, variables);

        // Success Confirmed check Item

        let mut variables = base_variables.clone();
        variables.id = Uuid::new_v4().to_string();
        variables.invoice_id_isil = confirmed_supplier_invoice.id.clone();

        let query = Insert::build_query(variables.clone());

        let response: Response<insert::ResponseData> = get_gql_result(&settings, query).await;
        let line = assert_unwrap_line!(response);

        assert_eq!(line.id, variables.id);

        let new_line = InvoiceLineRepository::new(&connection)
            .find_one_by_id(&line.id)
            .unwrap();

        assert_eq!(new_line.item_code, item.code);
        assert_eq!(new_line.item_name, item.name);

        // Check total calculation
        assert_eq!(
            new_line.total_after_tax,
            new_line.pack_size as f64
                * new_line.number_of_packs as f64
                * new_line.cost_price_per_pack
        );
    }

    impl PartialEq<insert::Variables> for InvoiceLineRow {
        fn eq(&self, other: &insert::Variables) -> bool {
            let insert::Variables {
                batch_isil,
                cost_price_per_pack_isil,
                expiry_date_isil,
                id: id_isil,
                invoice_id_isil,
                item_id_isil,
                number_of_packs_isil,
                sell_price_per_pack_isil,
                pack_size_isil,
            } = other;

            *cost_price_per_pack_isil == self.cost_price_per_pack
                && *expiry_date_isil == self.expiry_date
                && *id_isil == self.id
                && *invoice_id_isil == self.invoice_id
                && *item_id_isil == self.item_id
                && *number_of_packs_isil == self.number_of_packs as i64
                && *sell_price_per_pack_isil == self.sell_price_per_pack
                && *batch_isil == self.batch
                && *pack_size_isil == self.pack_size as i64
        }
    }

    impl PartialEq<insert::Variables> for StockLineRow {
        fn eq(&self, other: &insert::Variables) -> bool {
            let insert::Variables {
                batch_isil,
                cost_price_per_pack_isil,
                expiry_date_isil,
                id: _,
                invoice_id_isil: _,
                item_id_isil,
                number_of_packs_isil,
                sell_price_per_pack_isil,
                pack_size_isil,
            } = other;

            *cost_price_per_pack_isil == self.cost_price_per_pack
                && *expiry_date_isil == self.expiry_date
                && *item_id_isil == self.item_id
                && *number_of_packs_isil == self.available_number_of_packs as i64
                && *number_of_packs_isil == self.total_number_of_packs as i64
                && *sell_price_per_pack_isil == self.sell_price_per_pack
                && *batch_isil == self.batch
                && *pack_size_isil == self.pack_size as i64
        }
    }
}

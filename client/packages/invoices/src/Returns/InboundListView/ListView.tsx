import React, { FC } from 'react';
import {
  useNavigate,
  DataTable,
  useColumns,
  getNameAndColorColumn,
  TableProvider,
  createTableStore,
  useTranslation,
  InvoiceNodeStatus,
  NothingHere,
  useToggle,
  useUrlQueryParams,
} from '@openmsupply-client/common';
import { getStatusTranslator } from '../../utils';
import { Toolbar } from './Toolbar';
import { AppBarButtons } from './AppBarButtons';
import { InboundReturnRowFragment, useReturns } from '../api';

const InboundReturnListViewComponent: FC = () => {
  // TODO: for updating colour
  // const { mutate: onUpdate } = useReturns.document.updateInbound();
  const t = useTranslation('distribution');
  const {
    updateSortQuery,
    updatePaginationQuery,
    filter,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams({
    initialSort: { key: 'createdDatetime', dir: 'desc' },
    filters: [
      { key: 'otherPartyName' },
      { key: 'status', condition: 'equalTo' },
    ],
  });
  const navigate = useNavigate();
  const modalController = useToggle();
  const pagination = { page, first, offset };
  const queryParams = { ...filter, sortBy, first, offset };

  const { data, isError, isLoading } =
    useReturns.document.listInbound(queryParams);
  // useDisableInboundRows(data?.nodes); // see inbound shipment for implementation reference

  const columns = useColumns<InboundReturnRowFragment>(
    [
      // [getNameAndColorColumn(), { setter: onUpdate }],
      [getNameAndColorColumn(), { setter: () => {} }],
      [
        'status',
        {
          formatter: status =>
            getStatusTranslator(t)(status as InvoiceNodeStatus),
        },
      ],
      [
        'invoiceNumber',
        { description: 'description.invoice-number', maxWidth: 110 },
      ],
      'createdDatetime',
      'selection',
    ],
    { onChangeSortBy: updateSortQuery, sortBy },
    [sortBy]
  );

  return (
    <>
      <Toolbar filter={filter} />
      <AppBarButtons modalController={modalController} />

      <DataTable
        id="inbound-return-list"
        enableColumnSelection
        pagination={{ ...pagination, total: data?.totalCount ?? 0 }}
        onChangePage={updatePaginationQuery}
        columns={columns}
        data={data?.nodes ?? []}
        isError={isError}
        isLoading={isLoading}
        noDataElement={
          <NothingHere
            body={t('error.no-inbound-returns')}
            onCreate={modalController.toggleOn}
          />
        }
        onRowClick={row => {
          navigate(String(row.invoiceNumber));
        }}
      />
    </>
  );
};

export const InboundReturnListView: FC = () => (
  <TableProvider createStore={createTableStore}>
    <InboundReturnListViewComponent />
  </TableProvider>
);

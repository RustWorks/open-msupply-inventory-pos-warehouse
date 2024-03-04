import {
  useColumns,
  GenericColumnKey,
  ColumnAlign,
  getCommentPopoverColumn,
  useUrlQueryParams,
  ColumnDescription,
  NumUtils,
  TooltipTextCell,
} from '@openmsupply-client/common';
import { ResponseLineFragment, useResponse } from './../api';
import {
  PackVariantQuantityCell,
  PackVariantSelectCell,
} from '@openmsupply-client/system';

export const useResponseColumns = () => {
  const {
    updateSortQuery,
    queryParams: { sortBy },
  } = useUrlQueryParams({ initialSort: { key: 'itemName', dir: 'asc' } });
  const { isRemoteAuthorisation } = useResponse.utils.isRemoteAuthorisation();
  const columnDefinitions: ColumnDescription<ResponseLineFragment>[] = [
    getCommentPopoverColumn(),
    [
      'itemCode',
      {
        accessor: ({ rowData }) => rowData.item.code,
        getSortValue: rowData => rowData.item.code,
        width: 125,
      },
    ],
    [
      'itemName',
      {
        Cell: TooltipTextCell,
        accessor: ({ rowData }) => rowData.item.name,
        getSortValue: rowData => rowData.item.name,
      },
    ],
    {
      key: 'packUnit',
      label: 'label.unit',
      align: ColumnAlign.Right,
      Cell: PackVariantSelectCell({
        getItemId: r => r.itemId,
        getUnitName: r => r.item.unitName || null,
      }),
    },
    [
      'stockOnHand',
      {
        label: 'label.our-soh',
        description: 'description.our-soh',
        sortable: false,
        Cell: PackVariantQuantityCell({
          getItemId: row => row.itemId,
          getQuantity: row =>
            NumUtils.round(row.itemStats.availableStockOnHand),
        }),
      },
    ],
    {
      key: 'customerStockOnHand',
      label: 'label.customer-soh',
      description: 'description.customer-soh',
      width: 100,
      align: ColumnAlign.Right,
      getSortValue: rowData =>
        rowData.linkedRequisitionLine?.itemStats?.availableStockOnHand ?? '',
      Cell: PackVariantQuantityCell({
        getItemId: row => row.itemId,
        getQuantity: row =>
          NumUtils.round(
            row?.linkedRequisitionLine?.itemStats.availableStockOnHand ?? 0
          ),
      }),
    },
    [
      'requestedQuantity',
      {
        getSortValue: rowData => rowData.requestedQuantity,
        Cell: PackVariantQuantityCell({
          getItemId: row => row.itemId,
          getQuantity: row => NumUtils.round(row.requestedQuantity ?? 0),
        }),
      },
    ],
  ];

  if (isRemoteAuthorisation) {
    columnDefinitions.push({
      key: 'approvedQuantity',
      label: 'label.approved-quantity',
      sortable: false,
      Cell: PackVariantQuantityCell({
        getItemId: row => row.itemId,
        getQuantity: row => NumUtils.round(row.approvedQuantity),
      }),
    });
    columnDefinitions.push({
      key: 'approvalComment',
      label: 'label.approval-comment',
      sortable: false,
    });
  }

  columnDefinitions.push([
    'supplyQuantity',
    {
      getSortValue: rowData => rowData.supplyQuantity,
      Cell: PackVariantQuantityCell({
        getItemId: row => row.itemId,
        getQuantity: row => NumUtils.round(row.supplyQuantity),
      }),
    },
  ]);
  columnDefinitions.push({
    label: 'label.remaining-to-supply',
    description: 'description.remaining-to-supply',
    key: 'remainingToSupply',
    align: ColumnAlign.Right,
    getSortValue: rowData => rowData.remainingQuantityToSupply,
    Cell: PackVariantQuantityCell({
      getItemId: row => row.itemId,
      getQuantity: row => NumUtils.round(row.remainingQuantityToSupply),
    }),
    width: 100,
  });
  columnDefinitions.push(GenericColumnKey.Selection);

  const columns = useColumns<ResponseLineFragment>(
    columnDefinitions,
    {
      onChangeSortBy: updateSortQuery,
      sortBy,
    },
    [updateSortQuery, sortBy]
  );

  return { columns, sortBy, onChangeSortBy: updateSortQuery };
};

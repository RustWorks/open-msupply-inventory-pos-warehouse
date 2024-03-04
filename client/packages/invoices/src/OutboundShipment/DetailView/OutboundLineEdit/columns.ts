import {
  useColumns,
  ColumnAlign,
  ExpiryDateCell,
  CheckCell,
  CurrencyCell,
  useCurrency,
  LocationCell,
  Column,
  NumberCell,
} from '@openmsupply-client/common';
import { DraftStockOutLine } from '../../../types';
import { PackQuantityCell, StockOutLineFragment } from '../../../StockOut';
import { PackVariantCell, usePackVariant } from '@openmsupply-client/system';

export const useOutboundLineEditColumns = ({
  onChange,
  unit,
  itemId = '',
}: {
  onChange: (key: string, value: number, packSize: number) => void;
  unit: string;
  itemId?: string;
}) => {
  const { numberOfPacksFromQuantity } = usePackVariant(itemId, null);
  const { c } = useCurrency();
  const columns = useColumns<DraftStockOutLine>(
    [
      [
        'batch',
        {
          accessor: ({ rowData }) => rowData.stockLine?.batch,
        },
      ],
      [
        'expiryDate',
        {
          Cell: ExpiryDateCell,
          width: 80,
        },
      ],
      [
        'location',
        {
          accessor: ({ rowData }) => rowData.location?.code,
          width: 70,
          Cell: LocationCell,
        },
      ],
      {
        label: 'label.on-hold',
        key: 'onHold',
        Cell: CheckCell,
        accessor: ({ rowData }) => rowData.stockLine?.onHold,
        align: ColumnAlign.Center,
        width: 80,
      },
      [
        'sellPricePerPack',
        {
          Cell: CurrencyCell,
          formatter: sellPrice => c(Number(sellPrice)).format(),
          width: 120,
        },
      ],
      {
        Cell: NumberCell,
        label: 'label.in-store',
        key: 'totalNumberOfPacks',
        align: ColumnAlign.Right,
        width: 80,
        accessor: ({ rowData }) => rowData.stockLine?.totalNumberOfPacks,
      },
      {
        Cell: NumberCell,
        label: 'label.available-packs',
        key: 'availableNumberOfPacks',
        align: ColumnAlign.Right,
        width: 85,
        accessor: ({ rowData }) => rowData.stockLine?.availableNumberOfPacks,
      },
      {
        key: 'packUnit',
        label: 'label.pack',
        sortable: false,
        Cell: PackVariantCell({
          getItemId: row => row?.item.id,
          getPackSizes: row => [row.packSize ?? 1],
          getUnitName: row => row?.item.unitName ?? null,
        }),
        width: 130,
      },
      [
        'unitQuantity',
        {
          label: 'label.unit-quantity-issued',
          labelProps: { unit },
          accessor: ({ rowData }) =>
            numberOfPacksFromQuantity(rowData.numberOfPacks * rowData.packSize),
          width: 120,
        },
      ],
      [
        'numberOfPacks',
        {
          Cell: PackQuantityCell,
          width: 120,
          label: 'label.pack-quantity-issued',
          setter: ({ packSize, id, numberOfPacks }) =>
            onChange(id, numberOfPacks ?? 0, packSize ?? 1),
        },
      ],
    ],
    {},
    [onChange]
  );

  return columns;
};

export const useExpansionColumns = (): Column<StockOutLineFragment>[] =>
  useColumns([
    'batch',
    'expiryDate',
    [
      'location',
      {
        accessor: ({ rowData }) => rowData.location?.code,
      },
    ],
    {
      key: 'packUnit',
      label: 'label.pack',
      sortable: false,
      Cell: PackVariantCell({
        getItemId: row => row?.item.id,
        getPackSizes: row => [row.packSize ?? 1],
        getUnitName: row => row?.item.unitName ?? null,
      }),
      width: 130,
    },
    'numberOfPacks',
    [
      'unitQuantity',
      {
        accessor: ({ rowData }) => rowData.packSize * rowData.numberOfPacks,
      },
    ],
    [
      'sellPricePerUnit',
      {
        accessor: ({ rowData }) => rowData.sellPricePerPack,
      },
    ],
  ]);

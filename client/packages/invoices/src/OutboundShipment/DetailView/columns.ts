import {
  useColumns,
  getRowExpandColumn,
  getNotePopoverColumn,
  ColumnAlign,
  GenericColumnKey,
  SortBy,
  Column,
  ArrayUtils,
  useCurrency,
  InvoiceLineNodeType,
  PositiveNumberCell,
  TooltipTextCell,
  useColumnUtils,
} from '@openmsupply-client/common';
import { StockOutLineFragment } from '../../StockOut';
import { StockOutItem } from '../../types';

interface UseOutboundColumnOptions {
  sortBy: SortBy<StockOutLineFragment | StockOutItem>;
  onChangeSortBy: (column: Column<StockOutLineFragment | StockOutItem>) => void;
}

const expansionColumn = getRowExpandColumn<
  StockOutLineFragment | StockOutItem
>();
const notePopoverColumn = getNotePopoverColumn<
  StockOutLineFragment | StockOutItem
>();

const isDefaultPlaceholderRow = (row: StockOutLineFragment) =>
  row.type === InvoiceLineNodeType.UnallocatedStock && !row.numberOfPacks;

const getPackSize = (row: StockOutLineFragment) =>
  isDefaultPlaceholderRow(row) ? '' : row.packSize;

const getNumberOfPacks = (row: StockOutLineFragment) =>
  isDefaultPlaceholderRow(row) ? '' : row.numberOfPacks;

const getUnitQuantity = (row: StockOutLineFragment) =>
  isDefaultPlaceholderRow(row) ? '' : row.packSize * row.numberOfPacks;

export const useOutboundColumns = ({
  sortBy,
  onChangeSortBy,
}: UseOutboundColumnOptions): Column<StockOutLineFragment | StockOutItem>[] => {
  const { c } = useCurrency();
  const { getColumnProperty, getColumnPropertyAsString } = useColumnUtils();

  return useColumns(
    [
      [
        notePopoverColumn,
        {
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const noteSections = lines
                .map(({ batch, note }) => ({
                  header: batch ?? '',
                  body: note ?? '',
                }))
                .filter(({ body }) => !!body);
              return noteSections.length ? noteSections : null;
            } else {
              return rowData.batch && rowData.note
                ? { header: rowData.batch, body: rowData.note }
                : null;
            }
          },
        },
      ],
      [
        'itemCode',
        {
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'item', 'code'], default: '' },
              { path: ['item', 'code'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'item', 'code'], default: '' },
              { path: ['item', 'code'], default: '' },
            ]),
        },
      ],
      [
        'itemName',
        {
          Cell: TooltipTextCell,
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'item', 'name'] },
              { path: ['item', 'name'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'item', 'name'] },
              { path: ['item', 'name'], default: '' },
            ]),
        },
      ],
      [
        'itemUnit',
        {
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'item', 'unitName'] },
              { path: ['item', 'unitName'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'item', 'unitName'] },
              { path: ['item', 'unitName'], default: '' },
            ]),
        },
      ],
      [
        'batch',
        {
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'batch'] },
              { path: ['batch'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'batch'] },
              { path: ['batch'] },
            ]),
        },
      ],
      [
        'expiryDate',
        {
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'expiryDate'] },
              { path: ['expiryDate'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'expiryDate'] },
              { path: ['expiryDate'] },
            ]),
        },
      ],
      [
        'location',
        {
          getSortValue: row =>
            getColumnPropertyAsString(row, [
              { path: ['lines', 'location', 'code'] },
              { path: ['location', 'code'], default: '' },
            ]),
          accessor: ({ rowData }) =>
            getColumnProperty(rowData, [
              { path: ['lines', 'location', 'code'] },
              { path: ['location', 'code'], default: '' },
            ]),
        },
      ],
      [
        'numberOfPacks',
        {
          Cell: PositiveNumberCell,
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const packSize = ArrayUtils.ifTheSameElseDefault(
                lines,
                'packSize',
                ''
              );
              if (packSize) {
                return lines.reduce(
                  (acc, value) => acc + value.numberOfPacks,
                  0
                );
              } else {
                return '';
              }
            } else {
              return getNumberOfPacks(row);
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const packSize = ArrayUtils.ifTheSameElseDefault(
                lines,
                'packSize',
                ''
              );
              if (packSize) {
                return lines.reduce(
                  (acc, value) => acc + value.numberOfPacks,
                  0
                );
              } else {
                return '';
              }
            } else {
              return getNumberOfPacks(rowData);
            }
          },
        },
      ],
      [
        'packSize',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              return (
                ArrayUtils.ifTheSameElseDefault(lines, 'packSize', '') ?? ''
              );
            } else {
              return getPackSize(row) ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.ifTheSameElseDefault(lines, 'packSize', '');
            } else {
              return getPackSize(rowData);
            }
          },
        },
      ],
      [
        'unitQuantity',
        {
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.getUnitQuantity(lines);
            } else {
              return getUnitQuantity(rowData);
            }
          },
          getSortValue: rowData => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.getUnitQuantity(lines);
            } else {
              return getUnitQuantity(rowData);
            }
          },
        },
      ],
      {
        label: 'label.unit-price',
        key: 'sellPricePerUnit',
        align: ColumnAlign.Right,
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            return c(
              Object.values(rowData.lines).reduce(
                (sum, batch) =>
                  sum + (batch.sellPricePerPack ?? 0) / batch.packSize,
                0
              )
            ).format();
          } else {
            if (isDefaultPlaceholderRow(rowData)) return '';
            return c(
              (rowData.sellPricePerPack ?? 0) / rowData.packSize
            ).format();
          }
        },
        getSortValue: rowData => {
          if ('lines' in rowData) {
            return c(
              Object.values(rowData.lines).reduce(
                (sum, batch) =>
                  sum + (batch.sellPricePerPack ?? 0) / batch.packSize,
                0
              )
            ).format();
          } else {
            return c(
              (rowData.sellPricePerPack ?? 0) / rowData.packSize
            ).format();
          }
        },
      },
      {
        label: 'label.line-total',
        key: 'lineTotal',
        align: ColumnAlign.Right,
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            return c(
              Object.values(rowData.lines).reduce(
                (sum, batch) =>
                  sum + batch.sellPricePerPack * batch.numberOfPacks,
                0
              )
            ).format();
          } else {
            if (isDefaultPlaceholderRow(rowData)) return '';

            const x = c(
              rowData.sellPricePerPack * rowData.numberOfPacks
            ).format();
            return x;
          }
        },
        getSortValue: row => {
          if ('lines' in row) {
            return c(
              Object.values(row.lines).reduce(
                (sum, batch) =>
                  sum + batch.sellPricePerPack * batch.numberOfPacks,
                0
              )
            ).format();
          } else {
            const x = c(row.sellPricePerPack * row.numberOfPacks).format();
            return x;
          }
        },
      },
      expansionColumn,
      GenericColumnKey.Selection,
    ],
    { onChangeSortBy, sortBy },
    [sortBy]
  );
};

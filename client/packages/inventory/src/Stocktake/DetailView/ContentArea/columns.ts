import {
  getRowExpandColumn,
  GenericColumnKey,
  useColumns,
  ColumnAlign,
  ArrayUtils,
  Column,
  SortBy,
  PositiveNumberCell,
  getLinesFromRow,
  TooltipTextCell,
  useTranslation,
  TypedTFunction,
  LocaleKey,
} from '@openmsupply-client/common';
import { InventoryAdjustmentReasonRowFragment } from '@openmsupply-client/system';
import { StocktakeSummaryItem } from '../../../types';
import { StocktakeLineFragment } from '../../api';
import { useStocktakeLineErrorContext } from '../../context';

interface UseStocktakeColumnOptions {
  sortBy: SortBy<StocktakeLineFragment | StocktakeSummaryItem>;
  onChangeSortBy: (
    column: Column<StocktakeLineFragment | StocktakeSummaryItem>
  ) => void;
}

const expandColumn = getRowExpandColumn<
  StocktakeLineFragment | StocktakeSummaryItem
>();

const getStocktakeReasons = (
  rowData: StocktakeLineFragment | StocktakeSummaryItem,
  t: TypedTFunction<LocaleKey>
) => {
  if ('lines' in rowData) {
    const { lines } = rowData;
    const inventoryAdjustmentReasons = lines
      .map(({ inventoryAdjustmentReason }) => inventoryAdjustmentReason)
      .filter(Boolean) as InventoryAdjustmentReasonRowFragment[];
    if (inventoryAdjustmentReasons.length !== 0) {
      return (
        ArrayUtils.ifTheSameElseDefault(
          inventoryAdjustmentReasons,
          'reason',
          t('multiple')
        ) ?? ''
      );
    } else {
      return '';
    }
  } else {
    return rowData.inventoryAdjustmentReason?.reason ?? '';
  }
};

export const useStocktakeColumns = ({
  sortBy,
  onChangeSortBy,
}: UseStocktakeColumnOptions): Column<
  StocktakeLineFragment | StocktakeSummaryItem
>[] => {
  const { getError } = useStocktakeLineErrorContext();
  const t = useTranslation();

  return useColumns<StocktakeLineFragment | StocktakeSummaryItem>(
    [
      [
        'itemCode',
        {
          getSortValue: row => {
            return row.item?.code ?? '';
          },
          accessor: ({ rowData }) => {
            return rowData.item?.code ?? '';
          },
        },
      ],
      [
        'itemName',
        {
          Cell: TooltipTextCell,
          getSortValue: row => {
            return row.item?.name ?? '';
          },
          accessor: ({ rowData }) => {
            return rowData.item?.name ?? '';
          },
        },
      ],
      [
        'itemUnit',
        {
          getSortValue: row => {
            return row.item?.unitName ?? '';
          },
          accessor: ({ rowData }) => {
            return rowData.item?.unitName ?? '';
          },
        },
      ],
      [
        'batch',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              return (
                ArrayUtils.ifTheSameElseDefault(
                  lines,
                  'batch',
                  t('multiple')
                ) ?? ''
              );
            } else {
              return row.batch ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.ifTheSameElseDefault(
                lines,
                'batch',
                t('multiple')
              );
            } else {
              return rowData.batch;
            }
          },
        },
      ],
      [
        'expiryDate',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const expiryDate =
                ArrayUtils.ifTheSameElseDefault(
                  lines,
                  'expiryDate',
                  t('multiple')
                ) ?? '';
              return expiryDate;
            } else {
              return row.expiryDate ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const expiryDate = ArrayUtils.ifTheSameElseDefault(
                lines,
                'expiryDate',
                t('multiple')
              );
              return expiryDate;
            } else {
              return rowData.expiryDate;
            }
          },
        },
      ],
      [
        'location',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const locations = row.lines.flatMap(({ location }) =>
                !!location ? [location] : []
              );
              if (locations.length !== 0) {
                return ArrayUtils.ifTheSameElseDefault(
                  locations,
                  'code',
                  t('multiple')
                );
              } else {
                return '';
              }
            } else {
              return row.location?.code ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const locations = rowData.lines.flatMap(({ location }) =>
                !!location ? [location] : []
              );

              if (locations.length !== 0) {
                return ArrayUtils.ifTheSameElseDefault(
                  locations,
                  'code',
                  t('multiple')
                );
              }
            } else {
              return rowData.location?.code ?? '';
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
                ArrayUtils.ifTheSameElseDefault(
                  lines,
                  'packSize',
                  t('multiple')
                ) ?? ''
              );
            } else {
              return row.packSize ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.ifTheSameElseDefault(
                lines,
                'packSize',
                t('multiple')
              );
            } else {
              return rowData.packSize;
            }
          },
        },
      ],
      {
        key: 'snapshotNumPacks',
        label: 'label.snapshot-num-of-packs',
        description: 'description.snapshot-num-of-packs',
        align: ColumnAlign.Right,
        Cell: PositiveNumberCell,
        getIsError: row =>
          getLinesFromRow(row).some(
            r => getError(r)?.__typename === 'SnapshotCountCurrentCountMismatch'
          ),
        getSortValue: row => {
          if ('lines' in row) {
            const { lines } = row;
            return (
              lines.reduce(
                (total, line) => total + line.snapshotNumberOfPacks,
                0
              ) ?? 0
            ).toString();
          } else {
            return row.snapshotNumberOfPacks ?? '';
          }
        },
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return (
              lines.reduce(
                (total, line) => total + line.snapshotNumberOfPacks,
                0
              ) ?? 0
            ).toString();
          } else {
            return rowData.snapshotNumberOfPacks;
          }
        },
      },
      {
        key: 'countedNumPacks',
        label: 'label.counted-num-of-packs',
        description: 'description.counted-num-of-packs',
        align: ColumnAlign.Right,
        Cell: PositiveNumberCell,
        getIsError: row =>
          getLinesFromRow(row).some(
            r => getError(r)?.__typename === 'SnapshotCountCurrentCountMismatch'
          ),
        getSortValue: row => {
          if ('lines' in row) {
            const { lines } = row;
            return (
              lines.reduce(
                (total, line) => total + (line.countedNumberOfPacks ?? 0),
                0
              ) ?? 0
            ).toString();
          } else {
            return row.countedNumberOfPacks ?? '';
          }
        },
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return (
              lines.reduce(
                (total, line) => total + (line.countedNumberOfPacks ?? 0),
                0
              ) ?? 0
            ).toString();
          } else {
            return rowData.countedNumberOfPacks;
          }
        },
      },
      {
        key: 'difference',
        label: 'label.difference',
        align: ColumnAlign.Right,
        getSortValue: row => {
          if ('lines' in row) {
            const { lines } = row;
            const total =
              lines.reduce(
                (total, line) =>
                  total +
                  (line.snapshotNumberOfPacks -
                    (line.countedNumberOfPacks ?? line.snapshotNumberOfPacks)),
                0
              ) ?? 0;
            return (total < 0 ? Math.abs(total) : -total).toString();
          } else {
            return (
              row.snapshotNumberOfPacks -
                (row.countedNumberOfPacks ?? row.snapshotNumberOfPacks) ?? ''
            );
          }
        },
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            const total =
              lines.reduce(
                (total, line) =>
                  total +
                  (line.snapshotNumberOfPacks -
                    (line.countedNumberOfPacks ?? line.snapshotNumberOfPacks)),
                0
              ) ?? 0;
            return (total < 0 ? Math.abs(total) : -total).toString();
          } else {
            return (
              (rowData.countedNumberOfPacks ?? rowData.snapshotNumberOfPacks) -
              rowData.snapshotNumberOfPacks
            );
          }
        },
      },
      {
        key: 'inventoryAdjustmentReason',
        label: 'label.reason',
        accessor: ({ rowData }) => getStocktakeReasons(rowData, t),
        getSortValue: rowData => getStocktakeReasons(rowData, t),
      },
      {
        key: 'comment',
        label: 'label.stocktake-comment',
        getSortValue: row => {
          if ('lines' in row) {
            const { lines } = row;
            return (
              ArrayUtils.ifTheSameElseDefault(
                lines,
                'comment',
                t('multiple')
              ) ?? ''
            );
          } else {
            return row.comment ?? '';
          }
        },
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.ifTheSameElseDefault(
              lines,
              'comment',
              t('multiple')
            );
          } else {
            return rowData.comment;
          }
        },
      },
      expandColumn,
      GenericColumnKey.Selection,
    ],
    { sortBy, onChangeSortBy },
    [sortBy, onChangeSortBy]
  );
};

export const useExpansionColumns = (): Column<StocktakeLineFragment>[] => {
  const { getError } = useStocktakeLineErrorContext();
  return useColumns([
    'batch',
    'expiryDate',
    [
      'location',
      {
        accessor: ({ rowData }) => rowData.location?.code,
      },
    ],
    'packSize',
    {
      key: 'snapshotNumPacks',
      width: 150,
      label: 'label.snapshot-num-of-packs',
      align: ColumnAlign.Right,
      getIsError: rowData =>
        getError(rowData)?.__typename === 'SnapshotCountCurrentCountMismatch',
      accessor: ({ rowData }) => rowData.snapshotNumberOfPacks,
    },
    {
      key: 'countedNumPacks',
      label: 'label.counted-num-of-packs',
      width: 150,
      align: ColumnAlign.Right,
      getIsError: rowData =>
        getError(rowData)?.__typename === 'StockLineReducedBelowZero',
      accessor: ({ rowData }) => rowData.countedNumberOfPacks,
    },
    'comment',
    {
      key: 'inventoryAdjustmentReason',
      label: 'label.reason',
      accessor: ({ rowData }) =>
        rowData.inventoryAdjustmentReason?.reason || '',
    },
  ]);
};

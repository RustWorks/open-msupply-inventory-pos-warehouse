import React, { FC, useEffect } from 'react';
import {
  TextInputCell,
  alpha,
  RecordPatch,
  DataTable,
  useColumns,
  CurrencyInputCell,
  useTranslation,
  getExpiryDateInputColumn,
  NonNegativeDecimalCell,
  EnabledCheckboxCell,
  ColumnDescription,
  Theme,
  useTheme,
  useTableStore,
  CellProps,
  getColumnLookupWithOverrides,
} from '@openmsupply-client/common';
import { DraftStocktakeLine } from './utils';
import {
  Adjustment,
  getLocationInputColumn,
  InventoryAdjustmentReasonRowFragment,
  InventoryAdjustmentReasonSearchInput,
  PackVariantEntryCell,
  usePackVariant,
} from '@openmsupply-client/system';
import {
  useStocktakeLineErrorContext,
  UseStocktakeLineErrors,
} from '../../../context';
import { StocktakeLineFragment } from '../../../api';

interface StocktakeLineEditTableProps {
  isDisabled?: boolean;
  batches: DraftStocktakeLine[];
  update: (patch: RecordPatch<DraftStocktakeLine>) => void;
}

const expiryDateColumn = getExpiryDateInputColumn<DraftStocktakeLine>();

type DraftLineSetter = (
  patch: Partial<DraftStocktakeLine> & { id: string }
) => void;

const useDisableStocktakeRows = (rows?: DraftStocktakeLine[]) => {
  const { setDisabledRows } = useTableStore();
  useEffect(() => {
    const disabledRows = rows
      ?.filter(row => !row.countThisLine)
      .map(({ id }) => id);
    if (disabledRows) setDisabledRows(disabledRows);
  }, [rows]);
};

const BatchInputCell = ({
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  isAutoFocus,
  ...props
}: CellProps<DraftStocktakeLine>): JSX.Element => (
  <TextInputCell {...props} isAutoFocus={props.rowIndex === 0} />
);

const getBatchColumn = (
  setter: DraftLineSetter,
  theme: Theme
): ColumnDescription<DraftStocktakeLine> =>
  [
    'batch',
    {
      width: 150,
      maxWidth: 150,
      maxLength: 50,
      Cell: BatchInputCell,
      setter: patch => setter({ ...patch, countThisLine: true }),
      backgroundColor: alpha(theme.palette.background.menu, 0.4),
      accessor: ({ rowData }) => rowData.batch || '',
    },
  ] as ColumnDescription<DraftStocktakeLine>;

const getCountThisLineColumn = (
  setter: DraftLineSetter,
  theme: Theme
): ColumnDescription<DraftStocktakeLine> => {
  return {
    key: 'countThisLine',
    label: 'label.count-this-line',
    width: 80,
    Cell: EnabledCheckboxCell,
    setter: patch => setter({ ...patch }),
    backgroundColor: alpha(theme.palette.background.menu, 0.4),
  };
};

const getInventoryAdjustmentReasonInputColumn = (
  setter: DraftLineSetter,
  { getError }: UseStocktakeLineErrors
): ColumnDescription<DraftStocktakeLine> => {
  return {
    key: 'inventoryAdjustmentReasonInput',
    label: 'label.reason',
    sortable: false,
    width: 120,
    accessor: ({ rowData }) => rowData.inventoryAdjustmentReason || '',
    Cell: ({ rowData, column, columnIndex, rowIndex }) => {
      const value = column.accessor({
        rowData,
      }) as InventoryAdjustmentReasonRowFragment | null;

      const onChange = (
        inventoryAdjustmentReason: InventoryAdjustmentReasonRowFragment | null
      ) => {
        setter({ ...rowData, inventoryAdjustmentReason });
      };

      const autoFocus = columnIndex === 0 && rowIndex === 0;

      const getAdjustment = () => {
        if (!rowData.countThisLine) return Adjustment.None;
        if (typeof rowData.countedNumberOfPacks !== 'number')
          return Adjustment.None;
        if (rowData.snapshotNumberOfPacks == rowData.countedNumberOfPacks)
          return Adjustment.None;
        if (rowData.snapshotNumberOfPacks > rowData.countedNumberOfPacks)
          return Adjustment.Reduction;

        return Adjustment.Addition;
      };

      const errorType = getError(rowData)?.__typename;
      const isAdjustmentReasonError =
        errorType === 'AdjustmentReasonNotProvided' ||
        errorType === 'AdjustmentReasonNotValid';

      // https://github.com/openmsupply/open-msupply/pull/1252#discussion_r1119577142, this would ideally live in inventory package
      // and instead of this method we do all of the logic in InventoryAdjustmentReasonSearchInput and use it in `Cell` field of the column
      return (
        <InventoryAdjustmentReasonSearchInput
          autoFocus={autoFocus}
          value={value}
          width={column.width}
          onChange={onChange}
          adjustment={getAdjustment()}
          isError={isAdjustmentReasonError}
        />
      );
    },
  };
};

// If this is not extracted to it's own component and used directly in Cell:
// cell will be re rendered anytime rowData changes, which causes it to loose focus
// if number of packs is changed and tab is pressed (in quick succession)
const PackUnitEntryCell = PackVariantEntryCell<DraftStocktakeLine>({
  getItemId: r => r.item.id,
  getUnitName: r => r.item.unitName || null,
  getIsDisabled: r => !r.isNewLine,
});

export const BatchTable: FC<
  StocktakeLineEditTableProps & { item: StocktakeLineFragment['item'] | null }
> = ({ item, batches, update, isDisabled = false }) => {
  const t = useTranslation('inventory');
  const theme = useTheme();
  const { packVariantExists } = usePackVariant(item?.id || '', null);
  useDisableStocktakeRows(batches);

  const errorsContext = useStocktakeLineErrorContext();

  const columns = useColumns<DraftStocktakeLine>([
    getCountThisLineColumn(update, theme),
    getBatchColumn(update, theme),
    {
      key: 'snapshotNumberOfPacks',
      label: 'label.snapshot-num-of-packs',
      width: 100,
      getIsError: rowData =>
        errorsContext.getError(rowData)?.__typename ===
        'SnapshotCountCurrentCountMismatch',
      setter: patch => update({ ...patch, countThisLine: true }),
      accessor: ({ rowData }) => rowData.snapshotNumberOfPacks || '0',
    },
    getColumnLookupWithOverrides('packSize', {
      label: packVariantExists
        ? 'label.unit-variant-and-pack-size'
        : 'label.pack-size',
      Cell: PackUnitEntryCell,
      setter: update,
    }),
    {
      key: 'countedNumberOfPacks',
      label: 'label.counted-num-of-packs',
      width: 100,
      getIsError: rowData =>
        errorsContext.getError(rowData)?.__typename ===
        'StockLineReducedBelowZero',
      Cell: NonNegativeDecimalCell,
      setter: patch => {
        // If counted number of packs was changed to result in no adjustment
        // we should remove inventoryAdjustmentReason, otherwise could have a reason
        // on a line with no adjustments
        const inventoryAdjustmentReason =
          !patch.countedNumberOfPacks ||
          patch.snapshotNumberOfPacks == patch.countedNumberOfPacks
            ? null
            : patch.inventoryAdjustmentReason;
        update({ ...patch, countThisLine: true, inventoryAdjustmentReason });
      },
      accessor: ({ rowData }) => rowData.countedNumberOfPacks ?? '',
    },
    [
      expiryDateColumn,
      {
        width: 140,
        setter: patch => update({ ...patch, countThisLine: true }),
      },
    ],
    getInventoryAdjustmentReasonInputColumn(update, errorsContext),
  ]);

  return (
    <DataTable
      id="stocktake-batch"
      isDisabled={isDisabled}
      columns={columns}
      data={batches}
      noDataMessage={t('label.add-new-line')}
      dense
    />
  );
};

export const PricingTable: FC<StocktakeLineEditTableProps> = ({
  batches,
  update,
  isDisabled,
}) => {
  const theme = useTheme();
  const t = useTranslation('inventory');
  const columns = useColumns<DraftStocktakeLine>([
    getCountThisLineColumn(update, theme),
    getBatchColumn(update, theme),
    [
      'sellPricePerPack',
      {
        Cell: CurrencyInputCell,
        width: 200,
        setter: patch => update({ ...patch, countThisLine: true }),
      },
    ],
    [
      'costPricePerPack',
      {
        Cell: CurrencyInputCell,
        width: 200,
        setter: patch => update({ ...patch, countThisLine: true }),
      },
    ],
  ]);

  return (
    <DataTable
      id="stocktake-pricing"
      isDisabled={isDisabled}
      columns={columns}
      data={batches}
      noDataMessage={t('label.add-new-line')}
      dense
    />
  );
};

export const LocationTable: FC<StocktakeLineEditTableProps> = ({
  batches,
  update,
  isDisabled,
}) => {
  const theme = useTheme();
  const t = useTranslation('inventory');
  const columns = useColumns<DraftStocktakeLine>([
    getCountThisLineColumn(update, theme),
    getBatchColumn(update, theme),
    [
      getLocationInputColumn(),
      {
        width: 300,
        setter: patch => update({ ...patch, countThisLine: true }),
      },
    ],
    [
      'comment',
      {
        label: 'label.stocktake-comment',
        Cell: TextInputCell,
        width: 200,
        setter: patch => update({ ...patch, countThisLine: true }),
        accessor: ({ rowData }) => rowData.comment || '',
      },
    ],
  ]);

  return (
    <DataTable
      id="stocktake-location"
      isDisabled={isDisabled}
      columns={columns}
      data={batches}
      noDataMessage={t('label.add-new-line')}
      dense
    />
  );
};

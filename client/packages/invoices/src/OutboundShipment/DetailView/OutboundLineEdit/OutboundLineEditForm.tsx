import React, { useCallback, useEffect, useState } from 'react';
import {
  Grid,
  BasicTextInput,
  ModalLabel,
  ModalRow,
  Select,
  useTranslation,
  Divider,
  Box,
  Typography,
  useFormatNumber,
  useDebounceCallback,
  NumericTextInput,
  useDebouncedValueCallback,
} from '@openmsupply-client/common';
import {
  ItemStockOnHandFragment,
  PackVariantSelect,
  StockItemSearchInput,
  usePackVariant,
} from '@openmsupply-client/system';
import { useOutbound } from '../../api';
import { DraftItem } from '../../..';
import {
  PackSizeController,
  StockOutAlert,
  StockOutAlerts,
  getAllocationAlerts,
} from '../../../StockOut';
import { DraftStockOutLine } from '../../../types';
import { isA } from '../../../utils';

interface OutboundLineEditFormProps {
  allocatedQuantity: number;
  availableQuantity: number;
  item: DraftItem | null;
  onChangeItem: (newItem: ItemStockOnHandFragment | null) => void;
  onChangeQuantity: (
    quantity: number,
    packSize: number | null,
    isAutoAllocated: boolean
  ) => DraftStockOutLine[] | undefined;
  packSizeController: PackSizeController;
  disabled: boolean;
  canAutoAllocate: boolean;
  isAutoAllocated: boolean;
  showZeroQuantityConfirmation: boolean;
  hasOnHold: boolean;
  hasExpired: boolean;
  setOkDisabled: (disabled: boolean) => void;
  draftStockOutLines: DraftStockOutLine[];
}

export const OutboundLineEditForm: React.FC<OutboundLineEditFormProps> = ({
  allocatedQuantity,
  onChangeItem,
  onChangeQuantity,
  item,
  packSizeController,
  availableQuantity,
  disabled,
  canAutoAllocate,
  isAutoAllocated,
  showZeroQuantityConfirmation,
  hasOnHold,
  hasExpired,
  setOkDisabled,
  draftStockOutLines,
}) => {
  const t = useTranslation('distribution');
  const [allocationAlerts, setAllocationAlerts] = useState<StockOutAlert[]>([]);
  const [issueQuantity, setIssueQuantity] = useState<number>();
  const { format } = useFormatNumber();
  const { items } = useOutbound.line.rows();

  const {
    variantsControl,
    activePackVariant,
    numberOfPacksFromQuantity,
    numberOfPacksToTotalQuantity,
  } = usePackVariant(item?.id ?? '', item?.unitName ?? null);

  const onChangePackSize = (newPackSize: number) => {
    const packSize = newPackSize === -1 ? 1 : newPackSize;
    const newAllocatedQuantity =
      newPackSize === 0 ? 0 : Math.round(allocatedQuantity / packSize);

    packSizeController.setPackSize(newPackSize);
    allocate(newAllocatedQuantity, newPackSize);
  };

  const updateIssueQuantity = useCallback(
    (quantity: number) => {
      setIssueQuantity(
        Math.round(
          quantity / Math.abs(Number(packSizeController.selected?.value || 1))
        )
      );
    },
    [packSizeController.selected?.value]
  );

  const debouncedSetAllocationAlerts = useDebounceCallback(
    warning => setAllocationAlerts(warning),
    []
  );

  const allocate = (quantity: number, packSize: number) => {
    const newAllocateQuantities = onChangeQuantity(
      quantity,
      packSize === -1 ? null : packSize,
      true
    );
    const placeholderLine = newAllocateQuantities?.find(isA.placeholderLine);
    const allocatedQuantity =
      newAllocateQuantities?.reduce(
        (acc, { numberOfPacks, packSize }) => acc + numberOfPacks * packSize,
        0
      ) ?? 0;
    const alerts = getAllocationAlerts(
      quantity * (packSize === -1 ? 1 : packSize),
      allocatedQuantity,
      placeholderLine?.numberOfPacks ?? 0,
      hasOnHold,
      hasExpired,
      format,
      t
    );
    debouncedSetAllocationAlerts(alerts);
    updateIssueQuantity(allocatedQuantity);
  };

  // using a debounced value for the allocation. In the scenario where
  // you have only pack sizes > 1 available, and try to type a quantity which starts with 1
  // e.g. 10, 12, 100.. then the allocation rounds the 1 up immediately to the available
  // pack size which stops you entering the required quantity.
  // See https://github.com/msupply-foundation/open-msupply/issues/2727
  const debouncedAllocate = useDebouncedValueCallback(
    (quantity, packSize) => {
      allocate(quantity, packSize);
      setOkDisabled(false);
    },
    [],
    500,
    [draftStockOutLines] // this is needed to prevent a captured enclosure of onChangeQuantity
  );

  const handleIssueQuantityChange = (quantity: number | undefined) => {
    setIssueQuantity(quantity ?? 0);
    setOkDisabled(true);
    debouncedAllocate(
      quantity ?? 0,
      Number(packSizeController.selected?.value)
    );
  };

  useEffect(() => {
    if (!isAutoAllocated) updateIssueQuantity(allocatedQuantity);
  }, [
    packSizeController.selected?.value,
    allocatedQuantity,
    isAutoAllocated,
    updateIssueQuantity,
  ]);

  console.log('OPTION', packSizeController.selected?.value);

  return (
    <Grid container gap="4px">
      <ModalRow>
        <ModalLabel label={t('label.item', { count: 1 })} />
        <Grid item flex={1}>
          <StockItemSearchInput
            autoFocus={!item}
            openOnFocus={!item}
            disabled={disabled}
            currentItemId={item?.id}
            onChange={onChangeItem}
            extraFilter={
              disabled
                ? undefined
                : item => !items?.some(({ id }) => id === item.id)
            }
          />
        </Grid>
      </ModalRow>
      {item && (
        <ModalRow>
          <ModalLabel label="" />
          <Grid item display="flex">
            <Typography
              sx={{
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
              }}
            >
              {t('label.available-quantity', {
                number: numberOfPacksFromQuantity(availableQuantity),
              })}
            </Typography>
          </Grid>

          <Grid style={{ display: 'flex' }} justifyContent="flex-end" flex={1}>
            <ModalLabel label={t('label.unit')} justifyContent="flex-end" />
            {variantsControl ? (
              <PackVariantSelect
                sx={{ minWidth: 150 }}
                variantControl={variantsControl}
              />
            ) : (
              <BasicTextInput
                disabled
                sx={{ width: 150 }}
                value={activePackVariant}
              />
            )}
          </Grid>
        </ModalRow>
      )}
      {item && canAutoAllocate ? (
        <>
          <Divider margin={10} />
          <StockOutAlerts
            allocationAlerts={allocationAlerts}
            showZeroQuantityConfirmation={showZeroQuantityConfirmation}
            isAutoAllocated={isAutoAllocated}
          />
          <Grid container>
            <ModalLabel label={t('label.issue')} />
            <NumericTextInput
              autoFocus
              value={
                packSizeController.selected?.value == -1
                  ? numberOfPacksFromQuantity(issueQuantity || 0)
                  : issueQuantity
              }
              onChange={quantity =>
                packSizeController.selected?.value == -1
                  ? handleIssueQuantityChange(
                      numberOfPacksToTotalQuantity(quantity || 0)
                    )
                  : handleIssueQuantityChange(quantity)
              }
            />
            <Box marginLeft={1} />

            {packSizeController.options.length ? (
              <>
                <Grid
                  item
                  alignItems="center"
                  display="flex"
                  justifyContent="flex-start"
                  style={{ minWidth: 125 }}
                >
                  <Select
                    sx={{ width: 110 }}
                    options={packSizeController.options}
                    value={packSizeController.selected?.value ?? ''}
                    clearable={false}
                    onChange={e => {
                      const { value } = e.target;
                      onChangePackSize(Number(value));
                    }}
                  />
                </Grid>
                <Box marginLeft="auto" />
              </>
            ) : null}
          </Grid>
        </>
      ) : (
        <Box height={100} />
      )}
    </Grid>
  );
};

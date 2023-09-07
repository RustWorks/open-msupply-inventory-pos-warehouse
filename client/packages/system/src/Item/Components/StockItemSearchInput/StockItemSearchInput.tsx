import React, { FC, useEffect, useMemo, useState } from 'react';
import {
  useToggle,
  useFormatNumber,
  useTranslation,
  AutocompleteWithPagination as Autocomplete,
  defaultOptionMapper,
  ArrayUtils,
  useDebounceCallback,
} from '@openmsupply-client/common';
import { ItemStockOnHandFragment, useItemStockOnHand } from '../../api';
import { itemFilterOptions, StockItemSearchInputProps } from '../../utils';
import { getItemOptionRenderer } from '../ItemOptionRenderer';
import { useItemFilter, usePagination } from './hooks';

const DEBOUNCE_TIMEOUT = 300;

export const StockItemSearchInput: FC<StockItemSearchInputProps> = ({
  onChange,
  currentItemId,
  disabled = false,
  extraFilter,
  width,
  autoFocus = false,
  openOnFocus,
}) => {
  const [items, setItems] = useState<ItemStockOnHandFragment[]>([]);
  const { pagination, onPageChange } = usePagination();
  const { filter, onFilter } = useItemFilter();

  const { data, isLoading } = useItemStockOnHand({
    pagination,
    filter,
  });
  const t = useTranslation('common');
  const formatNumber = useFormatNumber();

  const value = items.find(({ id }) => id === currentItemId) ?? null;
  const selectControl = useToggle();

  const options = useMemo(
    () =>
      defaultOptionMapper(
        extraFilter ? items.filter(extraFilter) ?? [] : items ?? [],
        'name'
      ),
    [items]
  );

  const cachedSearchedItems = useMemo(() => {
    const newItems = ArrayUtils.uniqBy(
      [...items, ...(data?.nodes ?? [])],
      'id'
    );
    const sorted = ArrayUtils.orderBy(newItems, ['name'], ['asc']);
    return sorted;
  }, [data]);

  const debounceOnFilter = useDebounceCallback(
    (searchText: string) => {
      onPageChange(0); // Reset pagination when searching for a new item
      onFilter(searchText);
    },
    [onFilter],
    DEBOUNCE_TIMEOUT
  );

  useEffect(() => {
    // using the Autocomplete openOnFocus prop, the popper is incorrectly positioned
    // when used within a Dialog. This is a workaround to fix the popper position.
    if (openOnFocus) {
      setTimeout(() => selectControl.toggleOn(), DEBOUNCE_TIMEOUT);
    }
  }, []);

  useEffect(() => setItems(cachedSearchedItems), [cachedSearchedItems]);

  return (
    <Autocomplete
      autoFocus={autoFocus}
      disabled={disabled}
      onOpen={selectControl.toggleOn}
      onClose={selectControl.toggleOff}
      filterOptionConfig={itemFilterOptions}
      loading={isLoading}
      value={value ? { ...value, label: value.name ?? '' } : null}
      noOptionsText={t('error.no-items')}
      onChange={(_, item) => onChange(item)}
      options={options}
      getOptionLabel={option => `${option.code}     ${option.name}`}
      renderOption={getItemOptionRenderer(
        t('label.units'),
        formatNumber.format
      )}
      width={width ? `${width}px` : '100%'}
      popperMinWidth={width}
      isOptionEqualToValue={(option, value) => option?.id === value?.id}
      open={selectControl.isOn}
      onInputChange={(_, value) => debounceOnFilter(value)}
      pagination={{ ...pagination, total: data?.totalCount ?? 0 }}
      paginationDebounce={DEBOUNCE_TIMEOUT}
      onPageChange={onPageChange}
    />
  );
};

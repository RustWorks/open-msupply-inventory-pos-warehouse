import { useMemo } from 'react';
import { SortUtils, useUrlQueryParams } from '@openmsupply-client/common';
import { useStocktakeColumns } from '../../../DetailView';
import { useStocktakeLines } from './useStocktakeLines';
import { useStocktakeItems } from './useStocktakeItems';

export const useStocktakeRows = (isGrouped = true) => {
  const {
    updateSortQuery,
    queryParams: { sortBy },
  } = useUrlQueryParams({ initialSort: { key: 'itemName', dir: 'desc' } });
  const { data: lines } = useStocktakeLines();
  const { data: items } = useStocktakeItems();
  const columns = useStocktakeColumns({
    onChangeSortBy: updateSortQuery,
    sortBy,
  });

  const sortedItems = useMemo(() => {
    const currentColumn = columns.find(({ key }) => key === sortBy.key);
    if (!currentColumn?.getSortValue) return items;
    const sorter = SortUtils.getColumnSorter(
      currentColumn?.getSortValue,
      !!sortBy.isDesc
    );
    return items?.sort(sorter);
  }, [items, sortBy.key, sortBy.isDesc]);

  const sortedLines = useMemo(() => {
    const currentColumn = columns.find(({ key }) => key === sortBy.key);
    if (!currentColumn?.getSortValue) return lines;
    const sorter = SortUtils.getColumnSorter(
      currentColumn?.getSortValue,
      !!sortBy.isDesc
    );
    return lines?.sort(sorter);
  }, [lines, sortBy.key, sortBy.isDesc]);

  const rows = isGrouped ? sortedItems : sortedLines;

  return {
    rows,
    lines: sortedLines,
    items: sortedItems,
    onChangeSortBy: updateSortQuery,
    sortBy,
  };
};

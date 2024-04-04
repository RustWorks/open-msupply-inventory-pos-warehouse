import {
  FilterByWithBoolean,
  SortBy,
  StockLineNode,
  StockLineSortFieldInput,
  useQuery,
} from '@openmsupply-client/common';
import { StockLineRowFragment } from '../../api/operations.generated';
import { useGraphQL } from '../useGraphQL';

export type ListParams = {
  first: number;
  offset: number;
  sortBy: SortBy<StockLineRowFragment>;
  filterBy: FilterByWithBoolean | null;
};

export const useStockList = (queryParams: ListParams) => {
  const { stockApi, storeId } = useGraphQL();

  const { sortBy, first, offset, filterBy } = queryParams;

  const queryKey = ['stock', storeId, 'list', sortBy, first, offset, filterBy];
  const queryFn = async (): Promise<{
    nodes: StockLineRowFragment[];
    totalCount: number;
  }> => {
    const filter = {
      ...filterBy,
      hasPacksInStore: true,
    };
    const result = await stockApi.stockLines({
      storeId,
      first: first,
      offset: offset,
      key: toSortField(sortBy),
      desc: sortBy.isDesc,
      filter,
    });
    const { nodes, totalCount } = result?.stockLines;
    return { nodes, totalCount };
  };

  const result = useQuery({ queryKey, queryFn });
  return result;
};

const toSortField = (
  sortBy: SortBy<StockLineNode>
): StockLineSortFieldInput => {
  switch (sortBy.key) {
    case 'batch':
      return StockLineSortFieldInput.Batch;
    case 'itemCode':
      return StockLineSortFieldInput.ItemCode;
    case 'itemName':
      return StockLineSortFieldInput.ItemName;
    case 'packSize':
      return StockLineSortFieldInput.PackSize;
    case 'supplierName':
      return StockLineSortFieldInput.SupplierName;
    case 'numberOfPacks':
      return StockLineSortFieldInput.NumberOfPacks;
    case 'location':
      return StockLineSortFieldInput.LocationCode;
    case 'expiryDate':
    default: {
      return StockLineSortFieldInput.ExpiryDate;
    }
  }
};

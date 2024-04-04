import { StockLineSortFieldInput, useQuery } from '@openmsupply-client/common';
import { StockLineRowFragment } from '../../api/operations.generated';
import { useGraphQL } from '../useGraphQL';

export const useExportStockList = () => {
  const { stockApi, storeId } = useGraphQL();

  const queryKey = ['stock', storeId, 'list'];
  const queryFn = async (): Promise<{
    nodes: StockLineRowFragment[];
    totalCount: number;
  }> => {
    const result = await stockApi.stockLines({
      key: StockLineSortFieldInput.ItemName,
      desc: false,
      storeId,
    });
    return result?.stockLines;
  };

  const { data, refetch, isLoading, isError } = useQuery({
    queryKey,
    queryFn,
    enabled: false,
  });
  return { data, fetchAllStock: refetch, isLoading, isError };
};

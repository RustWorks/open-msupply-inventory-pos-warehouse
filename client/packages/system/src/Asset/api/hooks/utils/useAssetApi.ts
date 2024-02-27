import { getAssetQueries, ListParams } from '../../api';
import { SortBy, useAuthContext, useGql } from '@openmsupply-client/common';
import { getSdk, AssetCatalogueItemFragment } from '../../operations.generated';

export const useAssetApi = () => {
  const { client } = useGql();
  const { storeId } = useAuthContext();

  const keys = {
    base: () => ['asset'] as const,
    detail: (id: string) => [...keys.base(), storeId, id] as const,
    list: () => [...keys.base(), storeId, 'list'] as const,
    paramList: (params: ListParams<AssetCatalogueItemFragment>) =>
      [...keys.list(), params] as const,
    sortedList: (sortBy: SortBy<AssetCatalogueItemFragment>) =>
      [...keys.list(), sortBy] as const,
  };

  const queries = getAssetQueries(getSdk(client), storeId);
  return { ...queries, storeId, keys };
};

import { useAuthContext, useGql } from '@openmsupply-client/common';
import { getSdk } from '../../operations.generated';
import { getSyncQueries } from '../../api';

export const useHostApi = () => {
  const keys = {
    base: () => ['host'] as const,
    displaySettings: () => [...keys.base(), 'displaySettings'] as const,
    syncSettings: () => [...keys.base(), 'syncSettings'] as const,
    syncStatus: () => [...keys.base(), 'syncStatus'] as const,
    syncInfo: () => [...keys.base(), 'syncStatus'] as const,
  };

  const { client } = useGql();
  const { storeId } = useAuthContext();
  const queries = getSyncQueries(getSdk(client));
  return { ...queries, storeId, keys };
};

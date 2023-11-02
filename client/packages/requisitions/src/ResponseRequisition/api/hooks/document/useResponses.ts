import { useQuery, useUrlQueryParams } from '@openmsupply-client/common';
import { useResponseApi } from '../utils/useResponseApi';

export const useResponses = () => {
  const { queryParams } = useUrlQueryParams({
    filters: [{ key: 'comment' }],
    initialSort: { key: 'createdDatetime', dir: 'desc' },
  });
  const api = useResponseApi();

  return {
    ...useQuery(api.keys.paramList(queryParams), () =>
      api.get.list(queryParams)
    ),
  };
};

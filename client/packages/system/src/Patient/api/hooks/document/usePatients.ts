import { useQuery, useUrlQueryParams } from '@openmsupply-client/common';
import { usePatientApi } from '../utils/usePatientApi';

export const usePatients = () => {
  const api = usePatientApi();
  const { queryParams } = useUrlQueryParams({
    filterKey: 'lastName',
    initialSort: { key: 'code', dir: 'asc' },
  });
  return {
    ...useQuery(api.keys.paramList(queryParams), () =>
      api.get.list(queryParams)
    ),
  };
};
import { useQuery, useUrlQueryParams } from '@openmsupply-client/common';
import { usePrescriptionApi } from '../../utils/usePrescriptionApi';

export const usePrescriptions = () => {
  const { queryParams } = useUrlQueryParams({
    filterKey: 'otherPartyName',
    initialSort: { key: 'createdDatetime', dir: 'desc' },
  });
  const api = usePrescriptionApi();

  return {
    ...useQuery(api.keys.paramList(queryParams), () =>
      api.get.list(queryParams)
    ),
  };
};

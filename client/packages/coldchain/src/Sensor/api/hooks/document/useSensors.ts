import { useUrlQueryParams } from '@common/hooks';
import { useSensorApi } from '../utils/useSensorApi';
import { useQuery } from 'packages/common/src';

export const useSensors = () => {
  const { queryParams } = useUrlQueryParams({
    initialSort: { key: 'serial', dir: 'desc' },
    filterKey: 'serial',
  });

  const api = useSensorApi();

  return {
    ...useQuery(api.keys.paramList(queryParams), api.get.list(queryParams)),
  };
};
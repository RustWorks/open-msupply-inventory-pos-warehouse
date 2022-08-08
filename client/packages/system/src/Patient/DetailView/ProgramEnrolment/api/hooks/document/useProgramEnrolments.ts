import { useQuery, useUrlQueryParams } from '@openmsupply-client/common';
import { usePatient } from '../../../../../api';
import { useProgramEnrolmentApi } from '../utils/useProgramEnrolmentApi';

export const useProgramEnrolments = () => {
  const api = useProgramEnrolmentApi();
  const { queryParams } = useUrlQueryParams({
    initialSort: { key: 'type', dir: 'asc' },
  });
  const patientId = usePatient.utils.id();
  const params = {
    ...queryParams,
    filterBy: { patientId: { equalTo: patientId } },
  };
  return {
    ...useQuery(api.keys.paramList(params), () => api.get.list(params)),
  };
};

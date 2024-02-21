import { useQuery } from '@openmsupply-client/common';
import { useReturnsApi } from '../utils/useReturnsApi';

export const useInboundReturnLines = (stockLineIds: string[]) => {
  const api = useReturnsApi();

  const { data } = useQuery(api.keys.newReturns(), () =>
    api.get.inboundReturnLines(stockLineIds)
  );

  return data;
};

import { useParams, useQuery } from '@openmsupply-client/common';
import { useReturnsApi } from '../utils/useReturnsApi';

export const useOutboundReturn = () => {
  const { invoiceNumber } = useParams();
  const api = useReturnsApi();

  return useQuery(api.keys.detail(), () =>
    api.get.invoiceByNumber(Number(invoiceNumber))
  );
};

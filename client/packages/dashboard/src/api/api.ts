import { getSdk } from './operations.generated';

export type DashboardQueries = ReturnType<typeof getSdk>;

export const getDashboardQueries = (
  queries: DashboardQueries,
  storeId: string
) => ({
  get: {
    stockCounts: async () => {
      const result = await queries.stockCounts({
        storeId,
        daysTillExpired: 30,
      });
      return {
        expired: result?.stockCounts.expired ?? 0,
        expiringSoon: result?.stockCounts.expiringSoon ?? 0,
      };
    },
    itemCounts: async (lowStockThreshold: number) => {
      const result = await queries.itemCounts({ storeId, lowStockThreshold });
      return result?.itemCounts?.itemCounts ?? {};
    },
    outboundShipmentCounts: async (): Promise<{
      notShipped: number;
    }> => {
      const result = await queries.outboundShipmentCounts({ storeId });
      return {
        notShipped: result?.invoiceCounts?.outbound.notShipped ?? 0,
      };
    },
    responseRequisitionCounts: async (): Promise<{
      newResponseRequisitionCount: number;
    }> => {
      const result = await queries.responseRequisitionCounts({ storeId });
      return {
        newResponseRequisitionCount:
          result?.requisitionCounts?.newResponseRequisitionCount ?? 0,
      };
    },
    inboundShipmentCounts: async (): Promise<{
      today: number;
      thisWeek: number;
      notDelivered: number;
    }> => {
      const result = await queries.inboundShipmentCounts({ storeId });

      return {
        thisWeek: result?.invoiceCounts?.inbound?.created?.thisWeek ?? 0,
        today: result?.invoiceCounts?.inbound?.created?.today ?? 0,
        notDelivered: result?.invoiceCounts?.inbound?.notDelivered ?? 0,
      };
    },
    requestRequisitionCounts: async (): Promise<{
      draftCount: number;
    }> => {
      const result = await queries.requestRequisitionCounts({ storeId });
      return {
        draftCount: result?.requestRequisitionCounts?.draftCount ?? 0,
      };
    },
  },
});

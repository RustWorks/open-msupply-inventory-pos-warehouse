import {
  InboundReturnInput,
  InvoiceNodeType,
  InvoiceSortFieldInput,
  OutboundReturnInput,
} from '@common/types';
import {
  InboundReturnRowFragment,
  OutboundReturnRowFragment,
  Sdk,
} from './operations.generated';
import { FilterByWithBoolean, SortBy } from '@common/hooks';

type ListParams<T> = {
  first: number;
  offset: number;
  sortBy: SortBy<T>;
  filterBy: FilterByWithBoolean | null;
};

export type OutboundListParams = ListParams<OutboundReturnRowFragment>;
export type InboundListParams = ListParams<InboundReturnRowFragment>;

const outboundParsers = {
  toSortField: (
    sortBy: SortBy<OutboundReturnRowFragment>
  ): InvoiceSortFieldInput => {
    switch (sortBy.key) {
      case 'createdDatetime': {
        return InvoiceSortFieldInput.CreatedDatetime;
      }
      case 'otherPartyName': {
        return InvoiceSortFieldInput.OtherPartyName;
      }
      case 'invoiceNumber': {
        return InvoiceSortFieldInput.InvoiceNumber;
      }
      case 'status':
      default: {
        return InvoiceSortFieldInput.Status;
      }
    }
  },
};
const inboundParsers = {
  toSortField: (
    sortBy: SortBy<InboundReturnRowFragment>
  ): InvoiceSortFieldInput => {
    switch (sortBy.key) {
      case 'createdDatetime': {
        return InvoiceSortFieldInput.CreatedDatetime;
      }
      case 'deliveredDatetime': {
        return InvoiceSortFieldInput.DeliveredDatetime;
      }
      case 'otherPartyName': {
        return InvoiceSortFieldInput.OtherPartyName;
      }
      case 'invoiceNumber': {
        return InvoiceSortFieldInput.InvoiceNumber;
      }
      case 'status':
      default: {
        return InvoiceSortFieldInput.Status;
      }
    }
  },
};

export const getReturnsQueries = (sdk: Sdk, storeId: string) => ({
  get: {
    listOutbound: async ({
      first,
      offset,
      sortBy,
      filterBy,
    }: OutboundListParams): Promise<{
      nodes: OutboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        ...filterBy,
        type: { equalTo: InvoiceNodeType.OutboundReturn },
      };
      const result = await sdk.outboundReturns({
        first,
        offset,
        key: outboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    listAllOutbound: async (
      sortBy: SortBy<OutboundReturnRowFragment>
    ): Promise<{
      nodes: OutboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        type: { equalTo: InvoiceNodeType.OutboundReturn },
      };
      const result = await sdk.outboundReturns({
        key: outboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    listInbound: async ({
      first,
      offset,
      sortBy,
      filterBy,
    }: InboundListParams): Promise<{
      nodes: InboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        ...filterBy,
        type: { equalTo: InvoiceNodeType.InboundReturn },
      };
      const result = await sdk.inboundReturns({
        first,
        offset,
        key: inboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    listAllInbound: async (
      sortBy: SortBy<InboundReturnRowFragment>
    ): Promise<{
      nodes: InboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        type: { equalTo: InvoiceNodeType.InboundReturn },
      };
      const result = await sdk.outboundReturns({
        key: inboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    outboundReturnLines: async (stockLineIds: string[]) => {
      const result = await sdk.generateOutboundReturnLines({
        stockLineIds,
        storeId,
      });

      return result?.generateOutboundReturnLines;
    },
    inboundReturnLines: async (stockLineIds: string[]) => {
      const result = await sdk.generateInboundReturnLines({
        stockLineIds,
        storeId,
      });

      return result?.generateInboundReturnLines;
    },
    outboundReturnByNumber: async (invoiceNumber: number) => {
      const result = await sdk.outboundReturnByNumber({
        invoiceNumber,
        storeId,
      });

      return result?.invoiceByNumber;
    },
  },
  insertOutboundReturn: async (input: OutboundReturnInput) => {
    const result = await sdk.insertOutboundReturn({
      input,
      storeId,
    });

    const { insertOutboundReturn } = result;

    if (insertOutboundReturn.__typename === 'InvoiceNode') {
      return insertOutboundReturn.invoiceNumber;
    }

    throw new Error('Could not insert outbound return');
  },
  insertInboundReturn: async (input: InboundReturnInput) => {
    const result = await sdk.insertInboundReturn({
      input,
      storeId,
    });

    const { insertInboundReturn } = result;

    if (insertInboundReturn.__typename === 'InvoiceNode') {
      return insertInboundReturn.invoiceNumber;
    }

    throw new Error('Could not insert inbound return');
  },
  deleteOutbound: async (
    returns: OutboundReturnRowFragment[]
  ): Promise<string[]> => {
    const result = await sdk.deleteOutboundReturns({
      storeId,
      input: {
        ids: returns.map(({ id }) => id),
      },
    });

    const { deleteOutboundReturns } = result;

    if (deleteOutboundReturns.__typename === 'DeletedIdsResponse') {
      return deleteOutboundReturns.deletedIds;
    }

    // TODO: query for and handle error response...
    throw new Error('Could not delete outbound returns');
  },
  deleteInbound: async (
    returns: InboundReturnRowFragment[]
  ): Promise<string[]> => {
    const result = await sdk.deleteInboundReturns({
      storeId,
      input: {
        ids: returns.map(({ id }) => id),
      },
    });

    const { deleteInboundReturns } = result;

    if (deleteInboundReturns.__typename === 'DeletedIdsResponse') {
      return deleteInboundReturns.deletedIds;
    }

    // TODO: query for and handle error response...
    throw new Error('Could not delete inbound returns');
  },
});

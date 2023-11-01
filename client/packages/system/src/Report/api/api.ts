import {
  EnvUtils,
  FilterBy,
  SortBy,
  PrintReportSortInput,
} from '@openmsupply-client/common';
import { ReportRowFragment, Sdk } from './operations.generated';
import { JsonData } from 'packages/programs/src';

export type ReportListParams = {
  filterBy: FilterBy | null;
  sortBy: SortBy<ReportRowFragment>;
  offset: number;
};

export const getReportQueries = (sdk: Sdk, storeId: string) => ({
  get: {
    list: async ({ filterBy, sortBy }: ReportListParams) => {
      const result = await sdk.reports({
        filter: filterBy,
        key: sortBy.key,
        desc: sortBy.isDesc,
        storeId,
      });

      return result?.reports || [];
    },
    print: async ({
      reportId,
      dataId,
      args,
      sort,
    }: {
      reportId: string;
      dataId?: string;
      args?: JsonData;
      sort?: PrintReportSortInput;
    }) => {
      const format = EnvUtils.printFormat;
      const result = await sdk.printReport({
        dataId,
        reportId,
        storeId,
        format,
        arguments: args,
        sort,
      });
      if (result?.printReport?.__typename === 'PrintReportNode') {
        return result.printReport.fileId;
      }

      throw new Error('Unable to print report');
    },
  },
});

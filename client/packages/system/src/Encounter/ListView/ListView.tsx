import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  createTableStore,
  NothingHere,
  useUrlQueryParams,
  useNavigate,
  createQueryParamsStore,
  EncounterSortFieldInput,
} from '@openmsupply-client/common';
import { useEncounterListColumns } from './columns';
import {
  EncounterFragmentWithStatus,
  useEncounterFragmentWithStatus,
} from '../utils';
import { EncounterFragment, useEncounter } from '@openmsupply-client/programs';

const EncounterListComponent: FC = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams();
  const { queryParams } = useUrlQueryParams();
  const { data, isError, isLoading } = useEncounter.document.list({
    ...queryParams,
  });
  const pagination = { page, first, offset };
  const navigate = useNavigate();
  const columns = useEncounterListColumns({
    onChangeSortBy: updateSortQuery,
    sortBy,
    includePatient: true,
  });
  const dataWithStatus: EncounterFragmentWithStatus[] | undefined =
    useEncounterFragmentWithStatus(data?.nodes);

  return (
    <DataTable
      id="name-list"
      pagination={{ ...pagination, total: data?.totalCount }}
      onChangePage={updatePaginationQuery}
      columns={columns}
      data={dataWithStatus}
      isLoading={isLoading}
      isError={isError}
      onRowClick={row => {
        navigate(String(row.id));
      }}
      noDataElement={<NothingHere />}
    />
  );
};

export const EncounterListView: FC = () => (
  <TableProvider
    createStore={createTableStore}
    queryParamsStore={createQueryParamsStore<EncounterFragment>({
      initialSortBy: {
        key: EncounterSortFieldInput.StartDatetime,
        isDesc: true,
      },
    })}
  >
    <EncounterListComponent />
  </TableProvider>
);

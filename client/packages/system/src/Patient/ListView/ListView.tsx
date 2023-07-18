import React, { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  NothingHere,
  useFormatDateTime,
  ColumnAlign,
  useAlertModal,
  useTranslation,
  useUrlQueryParams,
  ReadOnlyCheckboxCell,
  ColumnDataAccessor,
  useAuthContext,
  ColumnDescription,
} from '@openmsupply-client/common';
import { usePatient, PatientRowFragment } from '../api';
import { AppBarButtons } from './AppBarButtons';
import { Toolbar } from './Toolbar';
import { usePatientStore } from '@openmsupply-client/programs';
import { ChipTableCell } from '../Components';

const programEnrolmentLabelAccessor: ColumnDataAccessor<
  PatientRowFragment,
  string[]
> = ({ rowData }): string[] => {
  return rowData.programEnrolments.map(it => {
    const programEnrolmentId = it.programEnrolmentId
      ? ` (${it.programEnrolmentId})`
      : '';
    return `${it.document.documentRegistry?.name}${programEnrolmentId}`;
  });
};

const PatientListComponent: FC = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    filter,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams({ filterKey: ['firstName', 'lastName', 'identifier'] });
  const { store } = useAuthContext();

  const { setDocumentName } = usePatientStore();
  const { data, isError, isLoading } = usePatient.document.list();
  const pagination = { page, first, offset };
  const t = useTranslation('patients');
  const { localisedDate } = useFormatDateTime();
  const navigate = useNavigate();
  const alert = useAlertModal({
    title: t('error.something-wrong'),
    message: t('messages.no-patient-record'),
    onOk: () => {},
  });

  const columnDefinitions: ColumnDescription<PatientRowFragment>[] = [
    { key: 'code', label: 'label.patient-id' },
    { key: 'code2', label: 'label.patient-nuic' },
    {
      key: 'firstName',
      label: 'label.first-name',
    },
    {
      key: 'lastName',
      label: 'label.last-name',
    },
    {
      key: 'gender',
      label: 'label.gender',
    },
    {
      key: 'dateOfBirth',
      label: 'label.date-of-birth',
      width: 175,
      formatter: dateString =>
        dateString ? localisedDate((dateString as string) || '') : '',
    },
  ];

  if (store?.preferences.omProgramModule) {
    columnDefinitions.push({
      label: 'label.program-enrolments',
      key: 'programEnrolments',
      sortable: false,
      accessor: programEnrolmentLabelAccessor,
      Cell: ChipTableCell,
      maxWidth: 250,
    });
  }

  columnDefinitions.push({
    key: 'isDeceased',
    label: 'label.deceased',
    align: ColumnAlign.Right,
    Cell: ReadOnlyCheckboxCell,
    sortable: false,
  });

  const columns = useColumns<PatientRowFragment>(
    columnDefinitions,
    {
      onChangeSortBy: updateSortQuery,
      sortBy,
    },
    [updateSortQuery, sortBy]
  );

  return (
    <>
      <Toolbar filter={filter} />
      <AppBarButtons sortBy={sortBy} />
      <DataTable
        id="patients"
        pagination={{ ...pagination, total: data?.totalCount }}
        onChangePage={updatePaginationQuery}
        columns={columns}
        data={data?.nodes}
        isLoading={isLoading}
        isError={isError}
        onRowClick={row => {
          if (!row.id || !row.document?.name || !row.document?.type) {
            alert();
            return;
          }
          setDocumentName(row.document.name);
          navigate(String(row.id));
        }}
        noDataElement={<NothingHere />}
      />
    </>
  );
};

export const PatientListView: FC = () => (
  <TableProvider createStore={createTableStore}>
    <PatientListComponent />
  </TableProvider>
);

import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  NothingHere,
  createQueryParamsStore,
  useFormatDateTime,
  ColumnAlign,
  useTranslation,
  useQueryParamsStore,
  ProgramEnrolmentSortFieldInput,
} from '@openmsupply-client/common';
import { ProgramEventFragment } from '../api';
import { usePatientModalStore } from '../../hooks';
import { PatientModal } from '../../PatientView';
import { ProgramRowFragmentWithId, usePatient } from '../../api';

const programEventCellValue = (events: ProgramEventFragment[]) => {
  // just take the name of the first event
  return events[0]?.name ?? '';
};

const ProgramListComponent: FC = () => {
  const {
    sort: { sortBy, onChangeSortBy },
    pagination: { page, first, offset, onChangePage },
  } = useQueryParamsStore();

  const { data, isError, isLoading } = usePatient.document.programEnrolments({
    key: sortBy.key as ProgramEnrolmentSortFieldInput,
    isDesc: sortBy.isDesc,
  });
  const pagination = { page, first, offset };
  const { localisedDate } = useFormatDateTime();
  const t = useTranslation('patients');
  const { setEditModal: setEditingModal, setModal: selectModal } =
    usePatientModalStore();

  const columns = useColumns<ProgramRowFragmentWithId>(
    [
      {
        key: 'type',
        label: 'label.enrolment-program',
      },
      {
        key: 'programPatientId',
        label: 'label.enrolment-patient-id',
      },
      {
        key: 'events',
        label: 'label.program-status',
        formatter: events =>
          programEventCellValue(events as ProgramEventFragment[]),
        sortable: false,
      },
      {
        key: 'enrolmentDatetime',
        label: 'label.enrolment-datetime',
        align: ColumnAlign.Right,
        width: 175,
        formatter: dateString =>
          dateString ? localisedDate((dateString as string) || '') : '',
      },
    ],
    { onChangeSortBy, sortBy },
    [sortBy]
  );

  return (
    <DataTable
      id="program-enrolment-list"
      pagination={{ ...pagination, total: data?.totalCount }}
      onChangePage={onChangePage}
      columns={columns}
      data={data?.nodes}
      isLoading={isLoading}
      isError={isError}
      onRowClick={row => {
        setEditingModal(PatientModal.Program, row.type, row.name, row.type);
      }}
      noDataElement={
        <NothingHere
          onCreate={() => selectModal(PatientModal.ProgramSearch)}
          body={t('messages.no-programs')}
          buttonText={t('button.add-program')}
        />
      }
    />
  );
};

export const ProgramListView: FC = () => (
  <TableProvider
    createStore={createTableStore}
    queryParamsStore={createQueryParamsStore<ProgramRowFragmentWithId>({
      initialSortBy: { key: 'type' },
    })}
  >
    <ProgramListComponent />
  </TableProvider>
);
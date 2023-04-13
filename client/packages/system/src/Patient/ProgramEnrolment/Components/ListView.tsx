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
import {
  PatientModal,
  ProgramEnrolmentRowFragmentWithId,
  ProgramEventFragment,
  usePatientModalStore,
  useProgramEnrolments,
} from '@openmsupply-client/programs';
import { usePatient } from '../../api';
import { getStatusTranslation } from '../utils';
import { encounterEventCellValue } from 'packages/system/src/Encounter/ListView/columns';

const ProgramListComponent: FC = () => {
  const {
    sort: { sortBy, onChangeSortBy },
    pagination: { page, first, offset, onChangePage },
  } = useQueryParamsStore();

  const patientId = usePatient.utils.id();

  const { data, isError, isLoading } =
    useProgramEnrolments.document.programEnrolments({
      sortBy: {
        key: sortBy.key as ProgramEnrolmentSortFieldInput,
        isDesc: sortBy.isDesc,
      },
      filterBy: { patientId: { equalTo: patientId } },
    });
  const pagination = { page, first, offset };
  const { localisedDate } = useFormatDateTime();
  const t = useTranslation('patients');
  const { setEditModal: setEditingModal, setModal: selectModal } =
    usePatientModalStore();

  const columns = useColumns<ProgramEnrolmentRowFragmentWithId>(
    [
      {
        key: 'type',
        label: 'label.enrolment-program',
        accessor: row => row.rowData?.document?.documentRegistry?.name,
      },
      {
        key: 'programEnrolmentId',
        label: 'label.enrolment-patient-id',
      },
      {
        key: 'events',
        label: 'label.additional-info',
        sortable: false,
        formatter: events =>
          encounterEventCellValue((events as ProgramEventFragment[]) ?? []),
      },
      {
        key: 'status',
        label: 'label.program-status',
        accessor: row => t(getStatusTranslation(row.rowData?.status)),
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
        setEditingModal(
          PatientModal.Program,
          row.program,
          row.name,
          row.program
        );
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
    createStore={createTableStore()}
    queryParamsStore={createQueryParamsStore<ProgramEnrolmentRowFragmentWithId>(
      {
        initialSortBy: { key: 'type' },
      }
    )}
  >
    <ProgramListComponent />
  </TableProvider>
);

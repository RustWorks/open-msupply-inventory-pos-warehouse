import React, { FC, useState } from 'react';
import {
  AlertIcon,
  Autocomplete,
  BasicSpinner,
  Box,
  DatePickerInput,
  DialogButton,
  EncounterNodeStatus,
  InputWithLabelRow,
  RouteBuilder,
  Stack,
  TimePickerInput,
  Typography,
  useDialog,
  useNavigate,
  useNotification,
} from '@openmsupply-client/common';
import { DateUtils, useTranslation } from '@common/intl';
import {
  EncounterRegistryByProgram,
  PatientModal,
  useClinicians,
  usePatientModalStore,
  EncounterFragment,
  useEncounter,
} from '@openmsupply-client/programs';
import { usePatient } from '../api';
import { AppRoute } from 'packages/config/src';
import { EncounterSearchInput } from './EncounterSearchInput';
import { ClinicianFragment } from 'packages/programs/src/api/operations.generated';

type Encounter = Pick<
  EncounterFragment,
  'startDatetime' | 'endDatetime' | 'status'
>;

export const CreateEncounterModal: FC = () => {
  const patientId = usePatient.utils.id();
  const t = useTranslation('patients');
  const { current, setModal: selectModal } = usePatientModalStore();
  const [encounterRegistry, setEncounterRegistry] = useState<
    EncounterRegistryByProgram | undefined
  >();
  const [isError, setIsError] = useState(false);

  const [draft, setDraft] = useState<Encounter | undefined>(undefined);
  const navigate = useNavigate();
  const { error } = useNotification();

  const { data: clinicianData } = useClinicians.document.list({});
  const clinicians = clinicianData?.nodes ?? [];

  const handleSave = useEncounter.document.upsert(
    patientId,
    encounterRegistry?.program?.type ?? '',
    encounterRegistry?.encounter.documentType ?? ''
  );

  const reset = () => {
    selectModal(undefined);
    setEncounterRegistry(undefined);
    setDraft(undefined);
    setIsError(false);
  };

  const { Modal } = useDialog({
    isOpen: current === PatientModal.Encounter,
    onClose: reset,
  });

  const onChangeEncounter = (entry: EncounterRegistryByProgram) => {
    setIsError(false);
    setEncounterRegistry(entry);
  };

  const setStartDatetime = (date: Date | null): void => {
    if (!date) return;

    const startDatetime = DateUtils.formatRFC3339(date);

    setDraft({
      ...draft,
      startDatetime,
      status: DateUtils.isFuture(date)
        ? EncounterNodeStatus.Scheduled
        : draft?.status,
    });
  };

  const setEndDatetime = (date: Date | null): void => {
    const endDatetime = date ? DateUtils.formatRFC3339(date) : null;
    if (endDatetime && draft?.startDatetime)
      setDraft({ ...draft, endDatetime });
  };

  const setClinician = (clinicianIndex: number) => {
    setDraft({ ...draft, clinician: clinicians[clinicianIndex] });
  };

  return (
    <Modal
      title={t('label.new-encounter')}
      cancelButton={<DialogButton variant="cancel" onClick={reset} />}
      okButton={
        <DialogButton
          variant={'create'}
          disabled={draft === undefined}
          onClick={async () => {
            if (encounterRegistry !== undefined) {
              const { id } = await handleSave(
                draft,
                encounterRegistry.encounter.formSchemaId
              );
              if (!!id)
                navigate(
                  RouteBuilder.create(AppRoute.Dispensary)
                    .addPart(AppRoute.Encounter)
                    .addPart(id)
                    .build()
                );
              else error(t('error.encounter-not-created'))();
            }
            reset();
          }}
        />
      }
      width={700}
    >
      <React.Suspense fallback={<div />}>
        <Stack alignItems="flex-start" gap={1} sx={{ paddingLeft: '20px' }}>
          <InputWithLabelRow
            label={t('label.encounter')}
            Input={
              <EncounterSearchInput onChange={onChangeEncounter} value={null} />
            }
          />
          <RenderForm
            isError={isError}
            isLoading={false}
            isProgram={!!encounterRegistry}
            form={
              <>
                <InputWithLabelRow
                  label={t('label.visit-date')}
                  Input={
                    <DatePickerInput
                      value={DateUtils.getDateOrNull(
                        draft?.startDatetime ?? null
                      )}
                      onChange={setStartDatetime}
                    />
                  }
                />
                <InputWithLabelRow
                  label={t('label.visit-start')}
                  Input={
                    <TimePickerInput
                      value={DateUtils.getDateOrNull(
                        draft?.startDatetime ?? null
                      )}
                      onChange={setStartDatetime}
                    />
                  }
                />
                <InputWithLabelRow
                  label={t('label.visit-end')}
                  Input={
                    <TimePickerInput
                      value={DateUtils.getDateOrNull(
                        draft?.endDatetime ?? null
                      )}
                      disabled={draft?.startDatetime === undefined}
                      onChange={setEndDatetime}
                    />
                  }
                />
                <InputWithLabelRow
                  label={t('label.clinician')}
                  Input={
                    <Autocomplete
                      value={{
                        label: getClinicianName(draft?.clinician),
                        value: draft?.clinician,
                      }}
                      width={'200'}
                      onChange={e => setClinician(e.target.value)}
                      options={clinicians.map(clinician => ({
                        label: `${clinician.firstName} ${clinician.lastName}`,
                        value: clinician,
                      }))}
                    />
                  }
                />
              </>
            }
          />
        </Stack>
      </React.Suspense>
    </Modal>
  );
};

const RenderForm = ({
  form,
  isError,
  isLoading,
  isProgram,
}: {
  form: React.ReactNode;
  isError: boolean;
  isLoading: boolean;
  isProgram: boolean;
}) => {
  const t = useTranslation('common');
  if (!isProgram) return null;
  if (isError)
    return (
      <Box display="flex" gap={1} padding={3}>
        <AlertIcon color="error" />
        <Typography color="error">{t('error.unable-to-load-data')}</Typography>
      </Box>
    );
  if (isLoading) return <BasicSpinner />;

  return <>{form}</>;
};

export const getClinicianName = (clinician: ClinicianFragment | undefined) => {
  if (clinician === undefined) return '';
  return `${clinician.firstName} ${clinician.lastName}`;
};

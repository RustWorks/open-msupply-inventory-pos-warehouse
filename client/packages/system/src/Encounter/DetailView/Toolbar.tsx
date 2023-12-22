import React, { FC, ReactNode, useEffect, useState } from 'react';
import {
  AppBarContentPortal,
  Box,
  InputWithLabelRow,
  Grid,
  useTranslation,
  BasicTextInput,
  DatePickerInput,
  DateUtils,
  TimePickerInput,
  UserIcon,
  useFormatDateTime,
  ClinicianNode,
  EncounterNodeStatus,
  LocaleKey,
  TypedTFunction,
  Option,
  useIntlUtils,
  DocumentRegistryCategoryNode,
  DeleteIcon,
  SxProps,
  Theme,
} from '@openmsupply-client/common';
import {
  EncounterFragment,
  useDocumentRegistry,
} from '@openmsupply-client/programs';
import {
  ClinicianAutocompleteOption,
  ClinicianSearchInput,
} from '../../Clinician';
import { Clinician } from '../../Clinician/utils';
import { IconButton, Select } from '@common/components';
import { encounterStatusTranslation } from '../utils';

const encounterStatusOption = (
  status: EncounterNodeStatus,
  t: TypedTFunction<LocaleKey>
): Option => {
  return {
    label: encounterStatusTranslation(status, t),
    value: status,
  };
};

const Row = ({
  label,
  Input,
  sx,
}: {
  label: string;
  Input: ReactNode;
  sx?: SxProps<Theme>;
}) => (
  <InputWithLabelRow labelWidth="90px" label={label} Input={Input} sx={sx} />
);
interface ToolbarProps {
  onChange: (patch: Partial<EncounterFragment>) => void;
  onDelete: () => void;
  encounter: EncounterFragment;
}
export const Toolbar: FC<ToolbarProps> = ({
  encounter,
  onChange,
  onDelete,
}) => {
  const [status, setStatus] = useState<EncounterNodeStatus | undefined>(
    encounter.status ?? undefined
  );
  const [startDatetime, setStartDatetime] = useState<string | undefined>();
  const [endDatetime, setEndDatetime] = useState<string | undefined | null>();
  const t = useTranslation('dispensary');
  const { localisedDate } = useFormatDateTime();
  const { getLocalisedFullName } = useIntlUtils();
  const [clinician, setClinician] =
    useState<ClinicianAutocompleteOption | null>();
  const { data: programEnrolmentRegistry } =
    useDocumentRegistry.get.documentRegistries({
      filter: {
        category: { equalTo: DocumentRegistryCategoryNode.ProgramEnrolment },
        contextId: { equalTo: encounter?.contextId },
      },
    });

  useEffect(() => {
    setStatus(encounter.status ?? undefined);
    setStartDatetime(encounter.startDatetime);
    setEndDatetime(encounter.endDatetime);
    setClinician({
      label: getLocalisedFullName(
        encounter.clinician?.firstName,
        encounter.clinician?.lastName
      ),
      value: encounter.clinician as Clinician,
    });
  }, [encounter]);

  const { patient } = encounter;

  const statusOptions = [
    encounterStatusOption(EncounterNodeStatus.Pending, t),
    encounterStatusOption(EncounterNodeStatus.Visited, t),
    encounterStatusOption(EncounterNodeStatus.Cancelled, t),
  ];
  if (status === EncounterNodeStatus.Deleted) {
    statusOptions.push(encounterStatusOption(EncounterNodeStatus.Deleted, t));
  }

  return (
    <AppBarContentPortal sx={{ display: 'flex', flex: 1, marginBottom: 1 }}>
      <Grid
        container
        flexDirection="row"
        display="flex"
        flex={1}
        alignItems="center"
      >
        <Grid
          item
          sx={{
            alignItems: 'center',
            backgroundColor: 'background.menu',
            borderRadius: '50%',
            display: 'flex',
            height: '100px',
            justifyContent: 'center',
            marginRight: 2,
            width: '100px',
          }}
        >
          <Box>
            <UserIcon fontSize="large" style={{ flex: 1 }} />
          </Box>
        </Grid>
        <Grid item display="flex" flex={1}>
          <Box display="flex" flex={1} flexDirection="column" gap={0.5}>
            <Box display="flex" gap={1.5}>
              <Row
                label={t('label.patient')}
                Input={
                  <BasicTextInput disabled value={encounter?.patient.name} />
                }
              />
              <Row
                label={t('label.date-of-birth')}
                Input={
                  <BasicTextInput
                    disabled
                    value={localisedDate(patient.dateOfBirth ?? '')}
                  />
                }
              />

              <Row
                label={t('label.encounter-status')}
                sx={{ marginLeft: 'auto' }}
                Input={
                  <Select
                    fullWidth
                    onChange={event => {
                      const newStatus = event.target
                        .value as EncounterNodeStatus;
                      setStatus(newStatus);
                      onChange({
                        status: newStatus,
                      });
                    }}
                    options={statusOptions}
                    value={status}
                  />
                }
              />

              <IconButton
                icon={<DeleteIcon />}
                onClick={onDelete}
                label={t('label.delete')}
              />
            </Box>
            <Box display="flex" gap={1.5}>
              <Row
                label={t('label.program')}
                Input={
                  <BasicTextInput
                    disabled
                    value={programEnrolmentRegistry?.nodes?.[0]?.name ?? ''}
                  />
                }
              />
              <Row
                label={t('label.clinician')}
                Input={
                  <ClinicianSearchInput
                    onChange={clinician => {
                      setClinician(clinician);
                      onChange({
                        clinician: clinician?.value as ClinicianNode,
                      });
                    }}
                    clinicianLabel={clinician?.label || ''}
                    clinicianValue={clinician?.value}
                  />
                }
              />
            </Box>
            <Box display="flex" gap={1}>
              <Row
                label={t('label.visit-date')}
                Input={
                  <DatePickerInput
                    value={DateUtils.getDateOrNull(startDatetime ?? null)}
                    onChange={date => {
                      const startDatetime = DateUtils.formatRFC3339(date);
                      setStartDatetime(startDatetime);
                      onChange({
                        startDatetime,
                        endDatetime: endDatetime ?? undefined,
                      });
                    }}
                  />
                }
              />
              <InputWithLabelRow
                label={t('label.visit-start')}
                labelWidth="60px"
                Input={
                  <TimePickerInput
                    value={DateUtils.getDateOrNull(startDatetime ?? null)}
                    onChange={date => {
                      const startDatetime = date
                        ? DateUtils.formatRFC3339(date)
                        : undefined;
                      if (startDatetime) {
                        setStartDatetime(startDatetime);
                        onChange({
                          startDatetime,
                          endDatetime: endDatetime ?? undefined,
                        });
                      }
                    }}
                  />
                }
              />
              <InputWithLabelRow
                label={t('label.visit-end')}
                labelWidth="60px"
                Input={
                  <TimePickerInput
                    value={DateUtils.getDateOrNull(endDatetime ?? null)}
                    onChange={date => {
                      const endDatetime = date
                        ? DateUtils.formatRFC3339(date)
                        : undefined;
                      if (endDatetime) {
                        setEndDatetime(endDatetime);
                        onChange({ endDatetime });
                      }
                    }}
                  />
                }
              />
            </Box>
          </Box>
        </Grid>
      </Grid>
    </AppBarContentPortal>
  );
};

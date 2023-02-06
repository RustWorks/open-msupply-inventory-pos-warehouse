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
} from '@openmsupply-client/common';
import { EncounterFragment, useEncounter } from '@openmsupply-client/programs';
import { getClinicianName } from '../../Patient/Encounter';

const Row = ({ label, Input }: { label: string; Input: ReactNode }) => (
  <InputWithLabelRow labelWidth="90px" label={label} Input={Input} />
);
interface ToolbarProps {
  onChange: (patch: Partial<EncounterFragment>) => void;
}
export const Toolbar: FC<ToolbarProps> = ({ onChange }) => {
  const id = useEncounter.utils.idFromUrl();
  const { mutate: fetchEncounter, data: encounter } =
    useEncounter.document.byIdPromise(id);
  const [startDatetime, setStartDatetime] = useState<string | undefined>();
  const [endDatetime, setEndDatetime] = useState<string | undefined | null>();
  const t = useTranslation('patients');

  useEffect(() => fetchEncounter(), []);

  useEffect(() => {
    if (!encounter) return;

    setStartDatetime(encounter.startDatetime);
    setEndDatetime(encounter.endDatetime);
  }, [encounter]);

  if (!encounter) return null;

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
            <Box display="flex" gap={1}>
              <Row
                label={t('label.patient')}
                Input={
                  <BasicTextInput disabled value={encounter?.patient.name} />
                }
              />
              <Row
                label={t('label.clinician')}
                Input={
                  <BasicTextInput
                    disabled
                    value={getClinicianName(encounter?.document.data.clinician)}
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
                      const startDatetime = date
                        ? DateUtils.formatRFC3339(date)
                        : undefined;
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
                label={t('label.visit-end')}
                labelWidth="60px"
                Input={
                  <TimePickerInput
                    value={DateUtils.getDateOrNull(endDatetime ?? null)}
                    onChange={date => {
                      const endDatetime = date
                        ? DateUtils.formatRFC3339(date)
                        : undefined;
                      setEndDatetime(endDatetime);
                      onChange({ endDatetime });
                    }}
                  />
                }
              />
            </Box>
            <Row
              label={t('label.program')}
              Input={<BasicTextInput disabled value={encounter?.program} />}
            />
          </Box>
        </Grid>
      </Grid>
    </AppBarContentPortal>
  );
};
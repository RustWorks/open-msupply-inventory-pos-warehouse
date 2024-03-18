import React from 'react';
import {
  AutocompleteOption,
  BasicTextInput,
  DateTimePickerInput,
  InputWithLabelRow,
  TextField,
  Typography,
} from '@common/components';
import Autocomplete from '@mui/material/Autocomplete';
import { DateUtils, useFormatDateTime, useTranslation } from '@common/intl';
import { Box, Formatter } from '@openmsupply-client/common';
import { AssetFragment } from '../../api';
import { Status } from '../../Components';
import { translateReason } from '../../utils';
import { LocationIds } from '../DetailView';
interface SummaryProps {
  draft?: AssetFragment & LocationIds;
  onChange: (patch: Partial<AssetFragment & LocationIds>) => void;
  locations: {
    label: string;
    value: string;
  }[];
}

const Container = ({ children }: { children: React.ReactNode }) => (
  <Box
    display="flex"
    flex={1}
    flexDirection="column"
    alignContent="center"
    padding={4}
  >
    {children}
  </Box>
);

const Section = ({
  children,
  heading,
}: {
  children: React.ReactNode;
  heading: string;
}) => (
  <Box
    display="flex"
    flexDirection="column"
    padding={2}
    paddingRight={4}
    sx={{ maxWidth: '600px', width: '100%' }}
  >
    <Heading>{heading}</Heading>
    {children}
  </Box>
);

const Heading = ({ children }: { children: React.ReactNode }) => (
  <Typography
    sx={{
      marginLeft: '158px',
      fontSize: '20px',
      fontWeight: 'bold',
    }}
  >
    {children}
  </Typography>
);

const Row = ({
  children,
  label,
}: {
  children: React.ReactNode;
  label: string;
}) => (
  <Box paddingTop={1.5}>
    <InputWithLabelRow
      labelWidth="150px"
      label={label}
      labelProps={{
        sx: { fontSize: '16px', paddingRight: 2, textAlign: 'right' },
      }}
      Input={
        <Box sx={{}} flex={1}>
          {children}
        </Box>
      }
    />
  </Box>
);

export const Summary = ({ draft, onChange, locations }: SummaryProps) => {
  const t = useTranslation('coldchain');
  const { localisedDate } = useFormatDateTime();

  if (!draft) return null;

  const defaultLocations:
    | AutocompleteOption<{
        label: string;
        value: string;
      }>[]
    | null = draft.locations.nodes.map(location => ({
    label: location.code,
    value: location.id,
  }));

  return (
    <Box display="flex" flex={1}>
      <Container>
        <Section heading={t('heading.asset-identification')}>
          <Row label={t('label.category')}>
            <BasicTextInput
              value={draft.catalogueItem?.assetCategory?.name ?? ''}
              disabled
              fullWidth
            />
          </Row>
          <Row label={t('label.type')}>
            <BasicTextInput
              value={draft.catalogueItem?.assetType?.name ?? ''}
              disabled
              fullWidth
            />
          </Row>
          <Row label={t('label.serial')}>
            <BasicTextInput
              value={draft.serialNumber ?? ''}
              fullWidth
              onChange={e => onChange({ serialNumber: e.target.value })}
            />
          </Row>
          <Row label={t('label.installation-date')}>
            <DateTimePickerInput
              value={DateUtils.getDateOrNull(draft.installationDate)}
              format="P"
              onChange={date =>
                onChange({ installationDate: Formatter.naiveDate(date) })
              }
              textFieldProps={{ fullWidth: true }}
            />
          </Row>
          <Row label={t('label.replacement-date')}>
            <DateTimePickerInput
              value={DateUtils.getDateOrNull(draft.replacementDate)}
              format="P"
              onChange={date =>
                onChange({ replacementDate: Formatter.naiveDate(date) })
              }
              textFieldProps={{ fullWidth: true }}
            />
          </Row>
        </Section>
        <Section heading={t('heading.cold-chain')}>
          <Row label={t('label.cold-storage-location')}>
            {locations ? (
              <Autocomplete
                multiple
                id="multi-select-location"
                options={locations}
                getOptionLabel={option => option.label}
                defaultValue={defaultLocations}
                filterSelectedOptions
                onChange={(
                  _event,
                  newSelectedLocations: {
                    label: string;
                    value: string;
                  }[]
                ) => {
                  onChange({
                    locationIds: newSelectedLocations.map(
                      location => location.value
                    ),
                  });
                }}
                renderInput={params => (
                  <TextField
                    {...params}
                    label="Current Asset Locations"
                    placeholder="locations"
                  />
                )}
              />
            ) : null}
          </Row>
        </Section>
      </Container>
      <Box
        marginTop={4}
        marginBottom={4}
        sx={{
          borderColor: 'gray.light',
          borderWidth: 0,
          borderLeftWidth: 1,
          borderStyle: 'solid',
        }}
      ></Box>
      <Container>
        <Section heading={t('heading.functional-status')}>
          <Row label={t('label.current-status')}>
            <Box display="flex">
              <Status status={draft.statusLog?.status} />
            </Box>
          </Row>
          <Row label={t('label.last-updated')}>
            <BasicTextInput
              value={localisedDate(draft.statusLog?.logDatetime)}
              disabled
              fullWidth
            />
          </Row>
          <Row label={t('label.reason')}>
            <BasicTextInput
              value={translateReason(draft.statusLog?.reason, t)}
              disabled
              fullWidth
            />
          </Row>
        </Section>
        <Section heading={t('label.additional-info')}>
          <Row label={t('label.notes')}>
            <BasicTextInput
              value={draft.notes}
              onChange={e => onChange({ notes: e.target.value })}
              fullWidth
              multiline
              rows={4}
            />
          </Row>
        </Section>
      </Container>
    </Box>
  );
};

import React, { FC } from 'react';
import { SensorFragment } from '../api';
import { LocationSearchInput } from '@openmsupply-client/system/src';
import { useTranslation } from '@common/intl';
import { BasicTextInput, InputWithLabelRow } from '@common/components';
import { Box, Formatter, TextWithLabelRow } from '@openmsupply-client/common';

interface SensorLineFormProps {
  draft: SensorFragment;
  onUpdate: (patch: Partial<SensorFragment>) => void;
}

export const SensorLineForm: FC<SensorLineFormProps> = ({
  draft,
  onUpdate,
}) => {
  const t = useTranslation('coldchain');
  const textSx = { paddingLeft: 2 };
  const labelWrap = { sx: { whiteSpace: 'pre-wrap' } };
  const inputTextAlign = { sx: { textAlign: 'end' } };

  return (
    <Box display="flex" flexDirection="column" gap={2}>
      <InputWithLabelRow
        label={t('label.sensor-name')}
        labelProps={inputTextAlign}
        Input={
          <BasicTextInput
            value={draft.name ?? ''}
            onChange={e => onUpdate({ name: e.target.value })}
            disabled={!!draft.name}
          />
        }
      />
      <InputWithLabelRow
        label={t('label.location')}
        labelProps={inputTextAlign}
        Input={
          <LocationSearchInput
            autoFocus={false}
            disabled={false}
            value={draft.location ?? null}
            width={180}
            onChange={location => {
              onUpdate({ location });
            }}
          />
        }
      />
      <TextWithLabelRow sx={textSx} label={t('label.cce')} text={''} />
      <TextWithLabelRow
        sx={textSx}
        label={t('label.serial')}
        text={draft.serial ?? ''}
      />
      <TextWithLabelRow
        sx={textSx}
        label={t('label.battery-level')}
        text={`${draft.batteryLevel?.toString()}%` ?? ''}
      />
      <TextWithLabelRow
        sx={textSx}
        labelProps={labelWrap}
        label={t('label.last-reading')}
        text={
          draft.latestTemperatureLog?.nodes[0]?.temperature.toString() ?? ''
        }
      />
      <TextWithLabelRow
        sx={textSx}
        labelProps={labelWrap}
        label={t('label.last-record')}
        text={Formatter.csvDateTimeString(
          draft.latestTemperatureLog?.nodes[0]?.datetime
        )}
      />
    </Box>
  );
};

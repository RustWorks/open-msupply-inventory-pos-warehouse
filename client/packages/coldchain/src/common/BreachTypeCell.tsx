import React, { ReactElement } from 'react';
import { RecordWithId, TemperatureBreachNodeType } from '@common/types';
import {
  CellProps,
  SnowflakeIcon,
  SunIcon,
  Typography,
  useTranslation,
} from '@openmsupply-client/common';

export const BreachTypeCell = <T extends RecordWithId>({
  rowData,
  column,
}: CellProps<T>): ReactElement => {
  const t = useTranslation('coldchain');

  const breachType = column.accessor({
    rowData,
  }) as TemperatureBreachNodeType | null;
  const temperature = breachType?.split('_')[0];
  const type = breachType?.split('_')[1];

  return (
    <>
      {breachType && (
        <Typography
          sx={{
            color: temperature === 'HOT' ? 'warning.main' : 'secondary.dark',
            display: 'flex',
            gap: '2px',
          }}
        >
          {temperature === 'HOT' ? (
            <SunIcon sx={{ stroke: 'warning.main' }} />
          ) : (
            <SnowflakeIcon sx={{ stroke: 'secondary.dark' }} />
          )}
          {type === 'CUMULATIVE'
            ? t('label.cumulative')
            : t('label.consecutive')}
        </Typography>
      )}
    </>
  );
};

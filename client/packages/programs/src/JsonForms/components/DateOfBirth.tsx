import React, { useEffect } from 'react';
import {
  rankWith,
  ControlProps,
  uiTypeIs,
  composePaths,
} from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import {
  BaseDatePickerInput,
  DateUtils,
  NonNegativeIntegerInput,
  useFormatDateTime,
  useTranslation,
  FormLabel,
  Box,
  DetailInputWithLabelRow,
} from '@openmsupply-client/common';
import {
  FORM_LABEL_COLUMN_WIDTH,
  DefaultFormRowSx,
  FORM_GAP,
  FORM_LABEL_WIDTH,
} from '../common';

export const dateOfBirthTester = rankWith(10, uiTypeIs('DateOfBirth'));

const UIComponent = (props: ControlProps) => {
  const { data, handleChange, label, path } = props;
  const [age, setAge] = React.useState<number | string>('');
  const [dob, setDoB] = React.useState<Date | null>(null);
  const t = useTranslation('common');
  const dateFormatter = useFormatDateTime().customDate;

  const dobPath = composePaths(path, 'dateOfBirth');
  const estimatedPath = composePaths(path, 'dateOfBirthIsEstimated');
  const onChangeDoB = (dob: Date | null) => {
    const dateOfBirth = DateUtils.getDateOrNull(dob);
    // if dob is invalid, clear age and don't update the form data
    if (dateOfBirth === null) {
      setAge('');
      return;
    }
    setAge(DateUtils.age(dateOfBirth));
    setDoB(dateOfBirth);
    handleChange(dobPath, dateFormatter(dateOfBirth, 'yyyy-MM-dd'));
    handleChange(estimatedPath, false);
  };

  const onChangeAge = (newAge: number) => {
    const dob = DateUtils.startOfYear(DateUtils.addYears(new Date(), -newAge));
    setDoB(dob);
    handleChange(dobPath, dateFormatter(dob, 'yyyy-MM-dd'));
    handleChange(estimatedPath, true);
    setAge(newAge);
  };

  useEffect(() => {
    if (!data) return;
    const dob = DateUtils.getDateOrNull(data.dateOfBirth);
    setDoB(dob);
    if (dob === null) return;
    setAge(DateUtils.age(dob));
  }, [data]);

  if (!props.visible) {
    return null;
  }
  return (
    <DetailInputWithLabelRow
      sx={DefaultFormRowSx}
      label={label}
      labelWidthPercentage={FORM_LABEL_WIDTH}
      inputAlignment={'start'}
      Input={
        <Box display="flex" alignItems="center" gap={FORM_GAP} width="100%">
          <BaseDatePickerInput
            // undefined is displayed as "now" and null as unset
            value={dob ?? null}
            onChange={onChangeDoB}
            format="dd/MM/yyyy"
            slotProps={{ textField: { sx: { width: 135 } } }}
            disableFuture
            disabled={!props.enabled}
          />
          <Box
            flex={0}
            style={{ textAlign: 'end' }}
            flexBasis={FORM_LABEL_COLUMN_WIDTH}
          >
            <FormLabel sx={{ fontWeight: 'bold' }}>{t('label.age')}:</FormLabel>
          </Box>
          <Box flex={0}>
            <NonNegativeIntegerInput
              value={age}
              sx={{ width: 65 }}
              onChange={onChangeAge}
            />
          </Box>
        </Box>
      }
    />
  );
};

export const DateOfBirth = withJsonFormsControlProps(UIComponent);

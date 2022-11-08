import React, { useState } from 'react';
import { ControlProps, rankWith, schemaTypeIs } from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import {
  NumericTextInput,
  useDebounceCallback,
} from '@openmsupply-client/common';
import {
  FORM_INPUT_COLUMN_WIDTH,
  FORM_LABEL_COLUMN_WIDTH,
} from '../styleConstants';
import { Box } from '@mui/system';
import { FormLabel } from '@mui/material';

export const numberTester = rankWith(3, schemaTypeIs('number'));

const UIComponent = (props: ControlProps) => {
  const { data, handleChange, label, path, errors } = props;
  const [localData, setLocalData] = useState<number | undefined>(data);
  const onChange = useDebounceCallback(
    (value: number) => handleChange(path, value),
    [path]
  );
  const error = !!errors;

  if (!props.visible) {
    return null;
  }
  return (
    <Box
      display="flex"
      alignItems="center"
      gap={2}
      justifyContent="space-around"
      style={{ minWidth: 300 }}
      marginTop={1}
    >
      <Box style={{ textAlign: 'end' }} flexBasis={FORM_LABEL_COLUMN_WIDTH}>
        <FormLabel sx={{ fontWeight: 'bold' }}>{label}:</FormLabel>
      </Box>
      <Box flexBasis={FORM_INPUT_COLUMN_WIDTH}>
        <NumericTextInput
          type="number"
          InputProps={{
            sx: { '& .MuiInput-input': { textAlign: 'right' }, minWidth: 100 },
          }}
          onChange={value => {
            setLocalData(value);
            onChange(value);
          }}
          disabled={!props.enabled}
          error={error}
          helperText={errors}
          value={localData ?? ''}
        />
      </Box>
    </Box>
  );
};

export const NumberField = withJsonFormsControlProps(UIComponent);
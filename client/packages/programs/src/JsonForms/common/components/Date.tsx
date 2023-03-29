import React, { FC } from 'react';
import { rankWith, ControlProps, isDateControl } from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import { TextFieldProps } from '@mui/material';
import {
  BasicTextInput,
  DetailInputWithLabelRow,
  useFormatDateTime,
} from '@openmsupply-client/common';
import { FORM_LABEL_WIDTH } from '../styleConstants';
import { DatePicker, DatePickerProps } from '@mui/x-date-pickers';
import { z } from 'zod';
import { useZodOptionsValidation } from '../hooks/useZodOptionsValidation';

const Options = z
  .object({
    disableFuture: z.boolean().optional(),
  })
  .strict()
  .optional();

type Options = z.infer<typeof Options>;

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const DatePickerTextInput = ({ variant, ...props }: TextFieldProps) => (
  <BasicTextInput
    error={!!props.error}
    helperText={props.error}
    FormHelperTextProps={
      !!props.error ? { sx: { color: 'error.main' } } : undefined
    }
    {...props}
    variant="standard"
  />
);

export const BaseDatePickerInput: FC<
  Omit<DatePickerProps<Date, Date>, 'renderInput'> & { error: string }
> = props => (
  <DatePicker
    disabled={props.disabled}
    renderInput={DatePickerTextInput}
    {...props}
  />
);

export const dateTester = rankWith(5, isDateControl);

const UIComponent = (props: ControlProps) => {
  const { data, handleChange, label, path, uischema } = props;
  const dateFormatter = useFormatDateTime().customDate;
  const { errors: zErrors, options } = useZodOptionsValidation(
    Options,
    uischema.options
  );

  const disableFuture = options?.disableFuture ?? false;

  if (!props.visible) {
    return null;
  }
  return (
    <DetailInputWithLabelRow
      sx={{
        marginTop: 0.5,
        gap: 2,
        minWidth: '300px',
        justifyContent: 'space-around',
      }}
      label={label}
      labelWidthPercentage={FORM_LABEL_WIDTH}
      inputAlignment="start"
      Input={
        <BaseDatePickerInput
          // undefined is displayed as "now" and null as unset
          value={data ?? null}
          onChange={e => {
            if (e) handleChange(path, dateFormatter(e, 'yyyy-MM-dd'));
          }}
          inputFormat="dd/MM/yyyy"
          disabled={!props.enabled}
          error={props.errors ?? zErrors}
          disableFuture={disableFuture}
        />
      }
    />
  );
};

export const Date = withJsonFormsControlProps(UIComponent);

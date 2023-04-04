import React from 'react';
import { ControlProps, rankWith, uiTypeIs } from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import { Box, Divider, Typography } from '@openmsupply-client/common';
import { z } from 'zod';
import { useZodOptionsValidation } from '../hooks/useZodOptionsValidation';

const Options = z
  .object({
    margin: z.number().optional(),
    hideDivider: z.boolean().optional(),
  })
  .strict()
  .optional();

type Options = z.infer<typeof Options>;

export const spacerTester = rankWith(3, uiTypeIs('Spacer'));

const UIComponent = ({ uischema, errors, visible }: ControlProps) => {
  // Validates the option
  const { errors: zErrors, options: schemaOptions } = useZodOptionsValidation(
    Options,
    uischema.options
  );

  const error = !!errors || !!zErrors;
  const margin = schemaOptions?.margin ?? 20;
  const showDivider = !schemaOptions?.hideDivider;

  if (error)
    return (
      <>
        <Divider margin={margin} color="red" />
        <Typography color="red">
          {errors} {zErrors}
        </Typography>
      </>
    );

  if (!visible) {
    return null;
  }

  return showDivider ? (
    <Divider margin={margin} />
  ) : (
    <Box sx={{ height: `${margin * 2}px` }} />
  );
};

export const Spacer = withJsonFormsControlProps(UIComponent);

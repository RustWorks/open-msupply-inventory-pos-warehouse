import React, { useCallback, useMemo } from 'react';
import { ControlProps, rankWith, uiTypeIs } from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import {
  Box,
  useConfirmationModal,
  useMutation,
  useTranslation,
  BasicTextInput,
  Button,
  FormLabel,
  labelWithPunctuation,
} from '@openmsupply-client/common';
import {
  FORM_LABEL_COLUMN_WIDTH,
  FORM_INPUT_COLUMN_WIDTH,
  DefaultFormRowSx,
  JsonFormsConfig,
  useZodOptionsValidation,
  useDocument,
  useProgramEnrolments,
  FORM_GAP,
} from '@openmsupply-client/programs';
import { get as extractProperty } from 'lodash';
import { z } from 'zod';
import { useJsonForms } from '@jsonforms/react';
import { useJSONFormsCustomError } from '../common/hooks/useJSONFormsCustomError';
import { useDebouncedTextInput } from '../common/hooks/useDebouncedTextInput';

export const idGeneratorTester = rankWith(10, uiTypeIs('IdGenerator'));

type GeneratorOptions = {
  parts: Part[];
  /**
   * By default, after ID is first saved, a confirmation will be displayed whenever the user
   * subsequently clicks the "Generate" button.
   * This behaviour can be suppressed by setting this option to `false` (default `true`)
   */
  confirmRegenerate?: boolean;
  /**
   * Prevents the ID from *ever* being regenerated after first save (default `false`).
   */
  preventRegenerate?: boolean;
  /** If user can provide its own id */
  allowManualEntry?: boolean;
  /**
   * List of validations that should be applied to a generated or manual entered id.
   * If any validation fails for an autogenerated id generation is retried till a unique id is
   * found (as long as a new id is generated).
   */
  validations?: IdValidation[];
};

type IdValidation = {
  type: 'UniqueEnrolmentId';
};

/**
 * Declaration type to specify what to do with a given string.
 * Only one value should be set in a single StringMutation.
 * If multiple fields are set its undefined which one is applied.
 * To apply multiple string mutations multiple StringMutation are needed (mutations array in Part).
 */
type StringMutation = {
  /** Take first N chars from the string */
  firstNChars?: number;
  /** Take last N chars from the string */
  lastNChars?: number;
  /** Convert the string to upper case */
  toUpperCase?: boolean;
  /** Compares the string to the keys in the map and replaces it with the value of the map */
  mapping?: Record<string, string>;
  /**
   * Uses the pad string and "overlays" the input string on top,
   * e.g. padString = "00000" and input = "156" results in "00156"
   */
  padString?: string;
};

/** Takes a string from a field in the data */
type FieldPart = {
  type: 'Field';
  /** Name of the field (if type is FIELD) */
  field: string | string[];
  /** String mutations applied in sequence */
  mutations: StringMutation[];
};

/** Uses the store code */
type StoreCodePart = {
  type: 'StoreCode';
  /** String mutations applied in sequence */
  mutations: StringMutation[];
};

/** Uses the store name */
type StoreNamePart = {
  type: 'StoreName';
  /** String mutations applied in sequence */
  mutations: StringMutation[];
};

/** Uses a number counter */
type NumberPart = {
  type: 'Number';
  /** Name of the number counter (if type is NUMBER) */
  numberName: string;
  /** String mutations applied in sequence */
  mutations: StringMutation[];
};

type Part = FieldPart | StoreCodePart | StoreNamePart | NumberPart;

const StringMutation: z.ZodType<StringMutation> = z
  .object({
    firstNChars: z.number().optional(),
    lastNChars: z.number().optional(),
    toUpperCase: z.boolean().optional(),
    mapping: z.record(z.string()).optional(),
    padString: z.string().optional(),
  })
  .strict();

const Part: z.ZodType<Part> = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('Field'),
    field: z.union([z.string(), z.array(z.string())]),
    mutations: z.array(StringMutation),
  }),
  z.object({
    type: z.literal('StoreCode'),
    mutations: z.array(StringMutation),
  }),
  z.object({
    type: z.literal('StoreName'),
    mutations: z.array(StringMutation),
  }),
  z.object({
    type: z.literal('Number'),
    numberName: z.string(),
    mutations: z.array(StringMutation),
  }),
]);

const IdValidation: z.ZodType<IdValidation> = z
  .object({
    type: z.literal('UniqueEnrolmentId'),
  })
  .strict();

const GeneratorOptions: z.ZodType<GeneratorOptions> = z
  .object({
    parts: z.array(Part),
    confirmRegenerate: z.boolean().optional().default(true),
    preventRegenerate: z.boolean().optional().default(false),
    allowManualEntry: z.boolean().optional(),
    validations: z.array(IdValidation).optional(),
  })
  .strict();

const validateFields = (
  options: GeneratorOptions,
  data: Record<string, unknown>
): string | undefined => {
  for (const part of options.parts ?? []) {
    if (part.type !== 'Field') {
      continue;
    }
    const field = extractProperty(data, part.field);
    const fieldName =
      typeof part.field === 'string' ? part.field : part.field.join('.');
    if (field === undefined || field === '') {
      return `Missing required field: ${fieldName}`;
    }
  }
};

const mutateString = (value: string, part: StringMutation): string => {
  if (part.mapping) {
    const replacement = part.mapping[value];
    if (replacement !== undefined) {
      return replacement;
    }
  }
  if (part.firstNChars !== undefined) {
    return value.slice(0, part.firstNChars);
  }
  if (part.lastNChars !== undefined) {
    return value.slice(value.length - part.lastNChars);
  }
  if (part.toUpperCase) {
    return value.toLocaleUpperCase();
  }
  if (part.padString) {
    return value.padStart(part.padString.length, part.padString);
  }
  return value;
};

const valueFromPart = async (
  { data, config, allocateNumber }: Omit<GenerateIdInput, 'options'>,
  part: Part
): Promise<string | undefined> => {
  switch (part.type) {
    case 'Field': {
      const field = extractProperty(data, part.field);
      if (field === undefined) {
        return undefined;
      }
      return String(field);
    }
    case 'StoreCode': {
      return config.store?.code;
    }
    case 'StoreName': {
      return config.store?.name;
    }
    case 'Number': {
      return `${await allocateNumber(part.numberName)}`;
    }
  }
};

type GenerateIdInput = {
  options: GeneratorOptions;
  data: Record<string, unknown>;
  config: JsonFormsConfig;
  allocateNumber: (numberName: string) => Promise<number>;
};

const generateId = async (input: GenerateIdInput): Promise<string> => {
  let output = '';
  for (const part of input.options.parts ?? []) {
    const value = await valueFromPart(input, part);
    if (value === undefined) {
      continue;
    }
    // apply mutations in sequence
    output += part.mutations.reduce(
      (value: string, mutation: StringMutation) =>
        mutateString(value, mutation),
      value
    );
  }
  return output;
};

const useUniqueProgramEnrolmentIdValidation = () => {
  const { mutateAsync: fetchProgramEnrolments } =
    useProgramEnrolments.document.programEnrolmentsPromise();

  // returns error string if validation fails
  return async (
    id: string,
    documentName: string
  ): Promise<string | undefined> => {
    const result = await fetchProgramEnrolments({
      filterBy: {
        programEnrolmentId: { equalTo: id },
      },
    });
    if (result.totalCount === 0) {
      return undefined;
    }

    if (result.nodes[0]?.name === documentName) {
      return undefined;
    }
    return `Duplicated id: ${id}`;
  };
};

const UIComponent = (props: ControlProps) => {
  const { label, path, data, visible, handleChange, uischema, config } = props;
  const t = useTranslation(['patients', 'common']);
  const { core } = useJsonForms();
  const { mutateAsync: mutateGenerateId } = useMutation(
    async (input: GenerateIdInput): Promise<string> => generateId(input)
  );
  const { mutateAsync: allocateNumber } = useDocument.utils.allocateNumber();
  const validateUniqueProgramEnrolmentId =
    useUniqueProgramEnrolmentIdValidation();
  const { customError, setCustomError } = useJSONFormsCustomError(
    path,
    'UIGenerator'
  );

  const { data: savedData } = useDocument.get.documentByName(
    config.documentName
  );

  const { errors, options } = useZodOptionsValidation(
    GeneratorOptions,
    uischema.options
  );

  const savedDataField = extractProperty(savedData?.data ?? {}, path);
  const canGenerate = !savedDataField || !options?.preventRegenerate;
  const requireConfirmation = !options?.confirmRegenerate
    ? false
    : !!savedDataField;

  const validationError = useMemo(() => {
    if (!options || !core?.data) {
      return;
    }
    return validateFields(options, core?.data);
  }, [options, core?.data]);
  const error = !!validationError || !!errors;

  const manualUpdate = useCallback(
    async (value: string | undefined) => {
      if (!options) return;

      if (value) {
        const error = await validateId(options, value);
        setCustomError(error);
      }
      handleChange(path, value);
    },
    [options, path]
  );

  const { text, onChange } = useDebouncedTextInput(data, manualUpdate);

  const validateId = async (
    options: GeneratorOptions,
    id: string
  ): Promise<string | undefined> => {
    for (const validation of options.validations ?? []) {
      if (validation.type === 'UniqueEnrolmentId') {
        const error = await validateUniqueProgramEnrolmentId(
          id,
          config.documentName
        );
        if (error) {
          return error;
        }
      }
    }
    return undefined;
  };

  const generate = useCallback(async () => {
    if (!options) {
      return;
    }
    let id = undefined;
    while (true) {
      const nextId = await mutateGenerateId({
        options,
        data: core?.data,
        config,
        allocateNumber,
      });
      // run till we find a valid id or till the generated id didn't change
      const finished = !(await validateId(options, nextId)) || id === nextId;
      id = nextId;
      if (finished) break;
    }
    setCustomError(undefined);
    handleChange(path, id);
  }, [options, path, core?.data, handleChange]);

  const confirmRegenerate = useConfirmationModal({
    title: t('heading.are-you-sure'),
    message: t('messages.regenerate-id-confirm'),
    onConfirm: generate,
  });

  if (!visible) {
    return null;
  }
  return (
    <Box sx={DefaultFormRowSx}>
      <Box style={{ textAlign: 'end' }} flexBasis={FORM_LABEL_COLUMN_WIDTH}>
        <FormLabel sx={{ fontWeight: 'bold' }}>
          {labelWithPunctuation(label)}:
        </FormLabel>
      </Box>
      <Box
        flexBasis={FORM_INPUT_COLUMN_WIDTH}
        display="flex"
        alignItems="center"
        gap={FORM_GAP}
      >
        <BasicTextInput
          disabled={!props.enabled || !options?.allowManualEntry}
          value={text}
          style={{ flex: 1 }}
          helperText={errors ?? customError}
          onChange={e => onChange(e.target.value)}
          error={!!errors || !!customError}
        />

        <Box>
          <Button
            disabled={error || !canGenerate}
            onClick={requireConfirmation ? () => confirmRegenerate() : generate}
            variant="outlined"
          >
            {t('label.generate')}
          </Button>
        </Box>
      </Box>
    </Box>
  );
};

export const IdGenerator = withJsonFormsControlProps(UIComponent);

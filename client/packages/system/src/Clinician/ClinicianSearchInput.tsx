import { Autocomplete } from '@openmsupply-client/common';
import { ClinicianFragment, useClinicians } from '@openmsupply-client/programs';
import React from 'react';
import { FC } from 'react';
import { ClinicianAutocompleteOption, Clinician } from './utils';

interface ClinicianSearchInputProps {
  onChange: (clinician: ClinicianAutocompleteOption | null) => void;
  clinicianLabel: string;
  clinicianValue?: Clinician;
}

export const getClinicianName = (
  clinician: ClinicianFragment | Clinician | undefined
) => {
  return clinician === undefined
    ? ''
    : `${clinician?.firstName ?? ''} ${clinician?.lastName ?? ''}`.trim();
};

export const ClinicianSearchInput: FC<ClinicianSearchInputProps> = ({
  onChange,
  clinicianLabel,
  clinicianValue,
}) => {
  const { data } = useClinicians.document.list({});
  const clinicians: ClinicianFragment[] = data?.nodes ?? [];

  return (
    <Autocomplete
      value={{
        label: clinicianLabel,
        value: clinicianValue,
      }}
      width={'200'}
      onChange={(_, option) => {
        onChange(option);
      }}
      options={clinicians.map(
        (clinician): ClinicianAutocompleteOption => ({
          label: getClinicianName(clinician),
          value: {
            firstName: clinician.firstName ?? '',
            lastName: clinician.lastName ?? '',
            id: clinician.id,
          },
        })
      )}
    />
  );
};
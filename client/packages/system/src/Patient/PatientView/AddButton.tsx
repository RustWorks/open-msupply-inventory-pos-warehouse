import React, { useState } from 'react';
import {
  useTranslation,
  SplitButton,
  SplitButtonOption,
  PlusCircleIcon,
} from '@openmsupply-client/common';
import { usePatientModalStore } from '../hooks';
import { PatientModal } from '.';
import { ProgramSearchModal } from '../../Program/Components';
import { usePatient } from '../api';

export const AddButton = () => {
  const t = useTranslation('patients');
  const { current, setCurrent, setDocument, reset } = usePatientModalStore();
  const { data } = usePatient.document.programs();
  const options = [
    {
      value: PatientModal.Prescription,
      label: t('button.add-prescription'),
      isDisabled: false,
    },
    {
      value: PatientModal.ProgramSearch,
      label: t('button.add-program'),
      isDisabled: false,
    },
    {
      value: PatientModal.Encounter,
      label: t('button.add-encounter'),
      isDisabled: false,
    },
  ];

  const [selectedOption, setSelectedOption] = useState<
    SplitButtonOption<PatientModal>
  >(options[1] as SplitButtonOption<PatientModal>);

  const onClick = () => {
    reset();
    setCurrent(selectedOption?.value);
  };

  return (
    <>
      <SplitButton
        color="primary"
        options={options}
        selectedOption={selectedOption}
        onSelectOption={setSelectedOption}
        Icon={<PlusCircleIcon />}
        onClick={onClick}
      />
      <ProgramSearchModal
        disabledPrograms={data?.nodes?.map(program => program.type)}
        open={current === PatientModal.ProgramSearch}
        onClose={() => setCurrent(undefined)}
        onChange={async documentRegistry => {
          const createDocument = { data: {}, documentRegistry };
          setCurrent(undefined);
          setDocument({ type: documentRegistry.documentType, createDocument });
          setCurrent(PatientModal.Program);
        }}
      />
    </>
  );
};
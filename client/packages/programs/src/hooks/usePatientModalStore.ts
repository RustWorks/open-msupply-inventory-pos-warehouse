import { create } from '@openmsupply-client/common';
import { FormInputData } from '@openmsupply-client/programs';

interface PatientModalDocument {
  name?: string;
  patientId?: string;
  type?: string;
  createDocument?: FormInputData;
}

export enum PatientModal {
  Program = 'PROGRAM',
  ProgramSearch = 'PROGRAM_SEARCH',
  Encounter = 'ENCOUNTER',
}

/**
 * The state of the various modals used in the patient detail area
 * `current` is the active / displayed modal - set to `undefined` to hide them all
 * `documentName` and `documentType` define the JsonForm doc
 */
interface PatientModalState {
  current?: PatientModal;
  document?: PatientModalDocument;
  programType?: string;
  reset: () => void;
  /** Just set the modal, the modal has to figure out what to do by itself */
  setModal: (current?: PatientModal) => void;
  /** Modal state for editing an existing document */
  setEditModal: (
    current: PatientModal,
    documentType: string,
    documentName: string,
    programType: string
  ) => void;
  /** Modal state for creating a new document */
  setCreationModal: (
    current: PatientModal,
    documentType: string,
    createDocument: FormInputData,
    programType: string
  ) => void;
}

export const usePatientModalStore = create<PatientModalState>(set => ({
  current: undefined,
  document: undefined,
  reset: () =>
    set(state => ({
      ...state,
      current: undefined,
      document: undefined,
      programType: undefined,
    })),
  setModal: (current?: PatientModal) =>
    set(state => ({
      ...state,
      document: undefined,
      programType: undefined,
      current,
    })),

  setEditModal: (
    current: PatientModal,
    documentType: string,
    documentName: string,
    programType: string
  ) =>
    set(state => ({
      ...state,
      current: current,
      document: { type: documentType, name: documentName },
      programType: programType,
    })),

  setCreationModal: (
    current: PatientModal,
    documentType: string,
    createDocument: FormInputData,
    programType: string
  ) =>
    set(state => ({
      ...state,
      current: current,
      document: { type: documentType, createDocument },
      programType: programType,
    })),
}));

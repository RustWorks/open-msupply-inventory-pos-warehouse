import { usePatient } from './usePatient';
import { usePatients } from './usePatients';
import { usePatientsAll } from './usePatientsAll';
import { useInsertPatient } from './useInsertPatient';
import { useUpdatePatient } from './useUpdatePatient';
import { useInsertProgramPatient } from './useInsertProgramPatient';
import { useUpdateProgramPatient } from './useUpdateProgramPatient';
import { useDocument } from '@openmsupply-client/programs';

export const Document = {
  usePatient,
  usePatients,
  usePatientsAll,
  useInsertPatient,
  useUpdatePatient,
  useInsertProgramPatient,
  useUpdateProgramPatient,
  useDocumentHistory: useDocument.get.history,
};

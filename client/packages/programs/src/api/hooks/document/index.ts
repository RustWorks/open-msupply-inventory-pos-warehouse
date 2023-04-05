import { useDocumentByName } from './useDocumentByName';
import { useDocumentRegistryByContext } from './useDocumentRegistryByContext';
import { useDocumentHistory } from './useDocumentHistory';

import { useProgramRegistries } from './useProgramRegistries';
import { useEncounterRegistriesByPrograms } from './useEncounterRegistriesByPrograms';

import { usePatientDocument } from './usePatientDocument';
import { useAllocateNumber } from './useAllocateNumber';

import { useEncounterFields } from './useEncounterFields';
import { useEncounterById, useEncounterByIdPromise } from './useEncounterById';
import { useEncounterPrevious } from './useEncounterPrevious';

import {
  useProgramEnrolments,
  useProgramEnrolmentsPromise,
} from './useProgramEnrolments';
import { useInsertProgramEnrolment } from './useInsertProgramEnrolment';
import { useUpdateProgramEnrolment } from './useUpdateProgramEnrolment';

import { useEncounters } from './useEncounters';
import { useUpsertEncounter } from './useUpsertEncounter';

import { useClinicians } from './useClinicians';
import { useProgramEvents } from './useProgramEvents';
import { useFormSchemaByType } from './useFormSchemaByType';
import { useDocumentRegistryByType } from './useDocumentRegistriesByType';
import { useDocumentRegistries } from './useDocumentRegistries';
import { useEncounterByDocName } from './useEncounterByDocName';

export const Document = {
  useDocumentByName,
  usePatientDocument,
  useDocumentHistory,

  useDocumentRegistries,
  useDocumentRegistryByContext,
  useDocumentRegistryByType,
  useProgramRegistries,
  useEncounterRegistriesByPrograms,

  useAllocateNumber,

  useProgramEnrolments,
  useProgramEnrolmentsPromise,
  useInsertProgramEnrolment,
  useUpdateProgramEnrolment,

  useEncounters,
  useEncounterById,
  useEncounterByIdPromise,
  useEncounterByDocName,
  useEncounterFields,
  useEncounterPrevious,
  useUpsertEncounter,

  useClinicians,

  useFormSchemaByType,
  useProgramEvents,
};

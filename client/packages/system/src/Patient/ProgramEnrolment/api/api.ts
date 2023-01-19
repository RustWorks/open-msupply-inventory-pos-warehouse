import {
  InsertProgramEnrolmentInput,
  UpdateProgramEnrolmentInput,
} from '@openmsupply-client/common';
import { ProgramEnrolmentDocumentFragment, Sdk } from './operations.generated';

export const getProgramEnrolmentQueries = (sdk: Sdk, storeId: string) => ({
  at: new Date().toISOString(),
  insertProgramEnrolment: async (
    input: InsertProgramEnrolmentInput
  ): Promise<ProgramEnrolmentDocumentFragment> => {
    const result = await sdk.insertProgramEnrolment({
      storeId,
      input,
    });

    if (result.insertProgramEnrolment.__typename === 'ProgramEnrolmentNode') {
      return result.insertProgramEnrolment.document;
    }

    throw new Error('Could not insert program');
  },

  updateProgramEnrolment: async (
    input: UpdateProgramEnrolmentInput
  ): Promise<ProgramEnrolmentDocumentFragment> => {
    const result = await sdk.updateProgramEnrolment({
      storeId,
      input,
    });

    if (result.updateProgramEnrolment.__typename === 'ProgramEnrolmentNode') {
      return result.updateProgramEnrolment.document;
    }

    throw new Error('Could not update program');
  },
});
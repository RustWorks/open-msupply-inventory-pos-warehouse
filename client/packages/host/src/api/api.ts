import {
  DisplaySettingsInput,
  DisplaySettingsHash,
} from '@openmsupply-client/common';

import { Sdk } from './operations.generated';

export const getHostQueries = (sdk: Sdk) => ({
  get: {
    displaySettings: async (input: DisplaySettingsHash) => {
      const result = await sdk.displaySettings({ input });
      return result.displaySettings;
    },
    plugins: async () => {
      const result = await sdk.plugins();
      if (Array.isArray(result?.plugins))
        return result?.plugins;

      throw new Error('Unable to fetch plugins');
    },
  },

  updateDisplaySettings: async (displaySettings: DisplaySettingsInput) => {
    const result = await sdk.updateDisplaySettings({ displaySettings });
    return result?.updateDisplaySettings;
  },
});

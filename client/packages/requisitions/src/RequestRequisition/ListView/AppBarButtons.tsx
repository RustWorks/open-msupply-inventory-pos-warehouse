import React, { FC } from 'react';

import {
  FnUtils,
  DownloadIcon,
  PlusCircleIcon,
  useNotification,
  AppBarButtonsPortal,
  ButtonWithIcon,
  Grid,
  useTranslation,
  FileUtils,
  SortBy,
  LoadingButton,
  ToggleState,
  Platform,
  EnvUtils,
} from '@openmsupply-client/common';
import { RequestRowFragment, useRequest } from '../api';
import { requestsToCsv } from '../../utils';
import { CreateRequisitionModal } from './CreateRequisitionModal';

export const AppBarButtons: FC<{
  modalController: ToggleState;
  sortBy: SortBy<RequestRowFragment>;
}> = ({ modalController, sortBy }) => {
  const { mutate: onCreate } = useRequest.document.insert();
  const { mutate: onProgramCreate } = useRequest.document.insertProgram();
  const { success, error } = useNotification();
  const t = useTranslation('common');
  const { isLoading, fetchAsync } = useRequest.document.listAll(sortBy);

  const csvExport = async () => {
    const data = await fetchAsync();
    if (!data || !data?.nodes.length) {
      error(t('error.no-data'))();
      return;
    }

    const csv = requestsToCsv(data.nodes, t);
    FileUtils.exportCSV(csv, t('filename.requests'));
    success(t('success'))();
  };

  return (
    <AppBarButtonsPortal>
      <Grid container gap={1}>
        <ButtonWithIcon
          Icon={<PlusCircleIcon />}
          label={t('label.new-requisition')}
          onClick={modalController.toggleOn}
        />
        <LoadingButton
          startIcon={<DownloadIcon />}
          variant="outlined"
          isLoading={isLoading}
          onClick={csvExport}
          disabled={EnvUtils.platform === Platform.Android}
        >
          {t('button.export')}
        </LoadingButton>
      </Grid>
      <CreateRequisitionModal
        isOpen={modalController.isOn}
        onClose={modalController.toggleOff}
        onChange={async newRequistion => {
          modalController.toggleOff();
          switch (newRequistion.type) {
            case 'general':
              return onCreate({
                id: FnUtils.generateUUID(),
                otherPartyId: newRequistion.name.id,
              });
            case 'program':
              const { type: _, ...rest } = newRequistion;
              return onProgramCreate({
                id: FnUtils.generateUUID(),
                ...rest,
              });
          }
        }}
      />
    </AppBarButtonsPortal>
  );
};

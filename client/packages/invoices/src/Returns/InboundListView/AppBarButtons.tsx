import React, { FC } from 'react';
import {
  DownloadIcon,
  PlusCircleIcon,
  AppBarButtonsPortal,
  ButtonWithIcon,
  Grid,
  useTranslation,
  LoadingButton,
  ToggleState,
  EnvUtils,
  Platform,
  useNotification,
  FileUtils,
} from '@openmsupply-client/common';
import { CustomerSearchModal } from '@openmsupply-client/system';
import { useReturns } from '../api';
import { inboundReturnsToCsv } from '../../utils';

export const AppBarButtonsComponent: FC<{
  modalController: ToggleState;
}> = ({ modalController }) => {
  const t = useTranslation('distribution');
  const { success, error } = useNotification();
  // const { mutate: onCreate } = useReturns.document.insertInbound();
  const { fetchAsync, isLoading } = useReturns.document.listAllInbound({
    key: 'createdDateTime',
    direction: 'desc',
    isDesc: true,
  });

  const csvExport = async () => {
    const data = await fetchAsync();
    if (!data || !data?.nodes.length) {
      error(t('error.no-data'))();
      return;
    }
    const csv = inboundReturnsToCsv(data.nodes, t);
    FileUtils.exportCSV(csv, t('filename.inbound-returns'));
    success(t('success'))();
  };

  return (
    <AppBarButtonsPortal>
      <CustomerSearchModal
        open={modalController.isOn}
        onClose={modalController.toggleOff}
        onChange={async name => {
          modalController.toggleOff();
          console.log('TODO: create. Selected customer:', name);
          // try {
          //   await onCreate({
          //     id: FnUtils.generateUUID(),
          //     otherPartyId: name?.id,
          //   });
          // } catch (e) {
          //   const errorSnack = error(
          //     'Failed to create return! ' + (e as Error).message
          //   );
          //   errorSnack();
          // }
        }}
      />
      <Grid container gap={1}>
        <ButtonWithIcon
          Icon={<PlusCircleIcon />}
          label={t('button.new-return')}
          onClick={modalController.toggleOn}
        />
        <LoadingButton
          startIcon={<DownloadIcon />}
          isLoading={isLoading}
          variant="outlined"
          onClick={csvExport}
          disabled={EnvUtils.platform === Platform.Android}
        >
          {t('button.export')}
        </LoadingButton>
      </Grid>
    </AppBarButtonsPortal>
  );
};

export const AppBarButtons = React.memo(AppBarButtonsComponent);

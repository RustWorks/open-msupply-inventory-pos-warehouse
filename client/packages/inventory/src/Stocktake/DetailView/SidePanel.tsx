import React, { FC } from 'react';
import {
  Grid,
  CopyIcon,
  DetailPanelAction,
  DetailPanelPortal,
  DetailPanelSection,
  PanelLabel,
  BufferedTextArea,
  useBufferState,
  useNotification,
  useTranslation,
  PanelRow,
  PanelField,
  InfoTooltipIcon,
  DeleteIcon,
  useSingleDeleteConfirmation,
} from '@openmsupply-client/common';
import { useStocktake } from '../api';
import { canDeleteStocktake } from '../../utils';

const AdditionalInfoSection: FC = () => {
  const t = useTranslation('common');

  const { comment, user, update } = useStocktake.document.fields([
    'comment',
    'user',
  ]);
  const [bufferedComment, setBufferedComment] = useBufferState(comment ?? '');
  const isDisabled = useStocktake.utils.isDisabled();

  return (
    <DetailPanelSection title={t('heading.additional-info')}>
      <Grid container gap={0.5} key="additional-info">
        <PanelRow>
          <PanelLabel>{t('label.entered-by')}</PanelLabel>
          <PanelField>{user?.username}</PanelField>
          {user?.email ? <InfoTooltipIcon title={user.email} /> : null}
        </PanelRow>

        <PanelLabel>{t('heading.comment')}</PanelLabel>
        <BufferedTextArea
          disabled={isDisabled}
          onChange={e => {
            setBufferedComment(e.target.value);
            update({ comment: e.target.value });
          }}
          value={bufferedComment}
        />
      </Grid>
    </DetailPanelSection>
  );
};

export const SidePanel: FC = () => {
  const { success } = useNotification();
  const t = useTranslation('inventory');
  const { data } = useStocktake.document.get();
  const { mutateAsync } = useStocktake.document.delete();
  const canDelete = data ? canDeleteStocktake(data) : false;

  const copyToClipboard = () => {
    navigator.clipboard
      .writeText(JSON.stringify(data, null, 4) ?? '')
      .then(() => success('Copied to clipboard successfully')());
  };

  const deleteAction = async () => {
    if (!data) return;
    await mutateAsync([data]);
  };

  const onDelete = useSingleDeleteConfirmation({
    deleteAction,
    messages: {
      confirmMessage: t('messages.confirm-delete-stocktake', {
        number: data?.stocktakeNumber,
      }),
      deleteSuccess: t('messages.deleted-stocktakes', {
        count: 1,
      }),
    },
  });

  return (
    <DetailPanelPortal
      Actions={
        <>
          <DetailPanelAction
            icon={<DeleteIcon />}
            title={t('label.delete')}
            onClick={onDelete}
            disabled={!canDelete}
          />
          <DetailPanelAction
            icon={<CopyIcon />}
            title={t('link.copy-to-clipboard')}
            onClick={copyToClipboard}
          />
        </>
      }
    >
      <AdditionalInfoSection />
    </DetailPanelPortal>
  );
};

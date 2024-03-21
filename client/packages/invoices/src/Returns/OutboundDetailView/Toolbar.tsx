import React, { FC } from 'react';
import {
  AppBarContentPortal,
  Box,
  InputWithLabelRow,
  BasicTextInput,
  Grid,
  DropdownMenu,
  useTranslation,
  DropdownMenuItem,
  DeleteIcon,
} from '@openmsupply-client/common';
import { useReturns } from '../api';

export const Toolbar: FC = () => {
  const t = useTranslation('replenishment');
  const onDelete = useReturns.document.deleteOutboundRows();
  const { data } = useReturns.document.outboundReturn();
  const { otherPartyName } = data ?? {};
  // const { isGrouped, toggleIsGrouped } = useIsGrouped('outboundShipment');
  //   const [theirReferenceBuffer, setTheirReferenceBuffer] =
  //     useBufferState(theirReference);
  //   const { mutateAsync: updateName } = useOutbound.document.updateName();

  //   const isDisabled = useOutbound.utils.isDisabled();

  return (
    <AppBarContentPortal sx={{ display: 'flex', flex: 1, marginBottom: 1 }}>
      <Grid
        container
        flexDirection="row"
        display="flex"
        flex={1}
        alignItems="flex-end"
      >
        <Grid item display="flex" flex={1}>
          <Box display="flex" flex={1} flexDirection="column" gap={1}>
            {otherPartyName && (
              <InputWithLabelRow
                label={t('label.supplier-name')}
                Input={<BasicTextInput value={otherPartyName} disabled />}
              />
            )}
            {/* <InputWithLabelRow
              label={t('label.customer-ref')}
              Input={
                <BasicTextInput
                  disabled={isDisabled}
                  size="small"
                  sx={{ width: 250 }}
                  value={theirReferenceBuffer ?? ''}
                  onChange={event => {
                    setTheirReferenceBuffer(event.target.value);
                    update({ theirReference: event.target.value });
                  }}
                />
              }
            /> */}
          </Box>
        </Grid>
        <Grid
          item
          display="flex"
          gap={1}
          justifyContent="flex-end"
          alignItems="center"
        >
          {/* <Box sx={{ marginRight: 2 }}>
            <Switch
              label={t('label.group-by-item')}
              onChange={toggleIsGrouped}
              checked={isGrouped}
              size="small"
              color="secondary"
            />
          </Box> */}
          <DropdownMenu label={t('label.actions')}>
            <DropdownMenuItem IconComponent={DeleteIcon} onClick={onDelete}>
              {t('button.delete-lines')}
            </DropdownMenuItem>
          </DropdownMenu>
        </Grid>
      </Grid>
    </AppBarContentPortal>
  );
};
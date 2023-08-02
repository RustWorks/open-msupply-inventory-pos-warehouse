import React, { FC } from 'react';
import {
  AppBarContentPortal,
  Grid,
  DropdownMenu,
  DropdownMenuItem,
  DeleteIcon,
  useTranslation,
  BufferedTextInput,
  useBufferState,
  InputWithLabelRow,
  DatePickerInput,
  Formatter,
  InfoPanel,
  SearchBar,
  useIsGrouped,
  Box,
  Switch,
  DateUtils,
} from '@openmsupply-client/common';
import { useStocktake } from '../api';

export const Toolbar: FC = () => {
  const t = useTranslation('inventory');
  const isDisabled = useStocktake.utils.isDisabled();
  const { isLocked, stocktakeDate, description, update } =
    useStocktake.document.fields(['isLocked', 'description', 'stocktakeDate']);
  const onDelete = useStocktake.line.deleteSelected();
  const { itemFilter, setItemFilter } = useStocktake.line.rows();
  const [descriptionBuffer, setDescriptionBuffer] = useBufferState(description);
  const { isGrouped, toggleIsGrouped } = useIsGrouped('stocktake');
  const infoMessage = isLocked
    ? t('messages.on-hold-stock-take')
    : t('messages.finalised-stock-take');

  return (
    <AppBarContentPortal sx={{ display: 'flex', flex: 1, marginBottom: 1 }}>
      <Grid
        container
        flexDirection="row"
        display="flex"
        flex={1}
        alignItems="flex-end"
      >
        <Grid item display="flex" flex={1} flexDirection="column" gap={1}>
          <InputWithLabelRow
            label={t('heading.description')}
            Input={
              <BufferedTextInput
                disabled={isDisabled}
                size="small"
                sx={{ width: 220 }}
                value={descriptionBuffer ?? ''}
                onChange={event => {
                  setDescriptionBuffer(event.target.value);
                  update({ description: event.target.value });
                }}
              />
            }
          />
          <InputWithLabelRow
            label={t('label.stocktake-date')}
            Input={
              <DatePickerInput
                disabled={isDisabled}
                value={DateUtils.getDateOrNull(stocktakeDate)}
                onChange={date => {
                  update({ stocktakeDate: Formatter.naiveDate(date) });
                }}
              />
            }
          />
          {isDisabled && <InfoPanel message={infoMessage} />}
        </Grid>

        <Grid
          item
          display="flex"
          gap={1}
          justifyContent="flex-end"
          alignItems="center"
        >
          <SearchBar
            placeholder={t('placeholder.filter-items')}
            value={itemFilter}
            onChange={newValue => {
              setItemFilter(newValue);
            }}
            debounceTime={0}
          />
          <Box sx={{ marginRight: 2 }}>
            <Switch
              label={t('label.group-by-item')}
              onChange={toggleIsGrouped}
              checked={isGrouped}
              size="small"
              color="secondary"
            />
          </Box>
          <DropdownMenu disabled={isDisabled} label={t('label.actions')}>
            <DropdownMenuItem IconComponent={DeleteIcon} onClick={onDelete}>
              {t('button.delete-lines')}
            </DropdownMenuItem>
          </DropdownMenu>
        </Grid>
      </Grid>
    </AppBarContentPortal>
  );
};

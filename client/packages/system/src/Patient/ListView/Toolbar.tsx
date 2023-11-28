import React, { FC } from 'react';
import {
  useTranslation,
  AppBarContentPortal,
  FilterMenu,
  FilterController,
  Box,
  GenderType,
} from '@openmsupply-client/common';

export const Toolbar: FC<{ filter: FilterController }> = () => {
  const t = useTranslation('dispensary');

  return (
    <AppBarContentPortal
      sx={{
        paddingBottom: '16px',
        flex: 1,
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <Box display="flex" gap={1}>
        <FilterMenu
          filters={[
            // Can instantiate multiple related filters at once using this
            // // "group" option
            // {
            //   type: 'group',
            //   name: 'Name',
            //   elements: [
            //     {
            //       type: 'text',
            //       name: t('label.first-name'),
            //       urlParameter: 'firstName',
            //       placeholder: t('placeholder.search-by-first-name'),
            //     },
            //     {
            //       type: 'text',
            //       name: t('label.last-name'),
            //       urlParameter: 'lastName',
            //       placeholder: t('placeholder.search-by-last-name'),
            //     },
            //   ],
            // },
            {
              type: 'text',
              name: t('label.first-name'),
              urlParameter: 'firstName',
              placeholder: t('placeholder.search-by-first-name'),
              isDefault: true,
            },
            {
              type: 'text',
              name: t('label.last-name'),
              urlParameter: 'lastName',
              placeholder: t('placeholder.search-by-last-name'),
              isDefault: true,
            },
            {
              type: 'text',
              name: t('label.patient-id'),
              urlParameter: 'identifier',
              placeholder: t('placeholder.search-by-identifier'),
              isDefault: true,
            },
            {
              type: 'date',
              name: t('label.date-of-birth'),
              urlParameter: 'dateOfBirth',
            },
            {
              type: 'enum',
              name: t('label.gender'),
              urlParameter: 'gender',
              options: [
                { label: 'Male', value: GenderType.Male },
                { label: 'Female', value: GenderType.Female },
              ],
            },
          ]}
        />
      </Box>
    </AppBarContentPortal>
  );
};

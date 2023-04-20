import React from 'react';
import {
  useNavigate,
  Select,
  MenuItem,
  Option,
} from '@openmsupply-client/common';
import { useIntlUtils, SupportedLocales, useUserName } from '@common/intl';

export const LanguageMenu: React.FC = () => {
  const navigate = useNavigate();
  const { i18n, setUserLocale, languageOptions } = useIntlUtils();
  const username = useUserName();
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = event.target;
    i18n.changeLanguage(value);
    setUserLocale(username, value as SupportedLocales);
    navigate(0);
  };

  const renderOption = (option: Option) => (
    <MenuItem
      key={option.value}
      value={option.value}
      sx={option.value === 'ar' ? { justifyContent: 'flex-end' } : {}}
    >
      {option.label}
    </MenuItem>
  );

  return (
    <Select
      onChange={handleChange}
      options={languageOptions}
      value={i18n.language}
      renderOption={renderOption}
    />
  );
};

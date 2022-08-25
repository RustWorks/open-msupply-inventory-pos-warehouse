import { IntlUtils } from '@common/intl';
import {
  addMinutes,
  addDays,
  addYears,
  isValid,
  differenceInMonths,
  differenceInYears,
  isPast,
  isThisWeek,
  isToday,
  isThisMonth,
  isAfter,
  isBefore,
  isEqual,
  format,
  parseISO,
  fromUnixTime,
  startOfToday,
  startOfDay,
  startOfYear,
  formatRFC3339,
} from 'date-fns';
import { enGB, enUS, fr, ar } from 'date-fns/locale';

// Map locale string (from i18n) to locale object (from date-fns)
const getLocaleObj = { fr, ar };

export const MINIMUM_EXPIRY_MONTHS = 3;

const dateInputHandler = (date: Date | string | number): Date => {
  // Assume a string is an ISO date-time string
  if (typeof date === 'string') return parseISO(date);
  // Assume a number is a UNIX timestamp
  if (typeof date === 'number') return fromUnixTime(date);
  return date as Date;
};

export const DateUtils = {
  addMinutes,
  addDays,
  addYears,
  getDateOrNull: (date: string | null): Date | null => {
    if (!date) return null;
    const maybeDate = new Date(date);
    return isValid(maybeDate) ? maybeDate : null;
  },
  isExpired: (expiryDate: Date): boolean => isPast(expiryDate),
  isAlmostExpired: (
    expiryDate: Date,
    threshold = MINIMUM_EXPIRY_MONTHS
  ): boolean => differenceInMonths(expiryDate, Date.now()) <= threshold,
  isThisWeek,
  isToday,
  isThisMonth,
  isAfter,
  isBefore,
  isEqual,
  isValid,
  age: (date: Date) => differenceInYears(startOfToday(), startOfDay(date)),
  startOfDay,
  startOfYear,
  formatRFC3339,
};

export const useFormatDateTime = () => {
  const language = IntlUtils.useCurrentLanguage();
  const locale =
    language === 'en'
      ? navigator.language === 'en-US'
        ? enUS
        : enGB
      : getLocaleObj[language];

  const localisedDate = (date: Date | string | number): string =>
    format(dateInputHandler(date), 'P', { locale });

  const localisedTime = (date: Date | string | number): string =>
    format(dateInputHandler(date), 'p', { locale });

  const localisedDateTime = (date: Date | string | number): string =>
    format(dateInputHandler(date), 'P p', { locale });

  const dayMonthShort = (date: Date | string | number): string =>
    format(dateInputHandler(date), 'dd MMM', { locale });

  const customDate = (
    date: Date | string | number,
    formatString: string
  ): string => format(dateInputHandler(date), formatString, { locale });

  return {
    localisedDate,
    localisedTime,
    localisedDateTime,
    dayMonthShort,
    customDate,
  };
};

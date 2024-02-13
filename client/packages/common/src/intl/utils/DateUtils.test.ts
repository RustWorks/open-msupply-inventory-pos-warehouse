import { renderHookWithProvider } from '../../utils/testing';
import { useFormatDateTime } from './DateUtils';

describe('useFormatDateTime', () => {
  it('to be truthy', () => {
    const truthy = true;
    expect(truthy).toBe(true);
  });
  it('getLocalDateTime returns start of day for local timezone regardless of time zone', () => {
    const { result } = renderHookWithProvider(useFormatDateTime);
    const timeZone = new Intl.DateTimeFormat().resolvedOptions().timeZone;
    const date = '2024-02-07';
    const options = {
      locale: {
        code: timeZone,
      },
    };
    expect(
      result.current
        .getLocalDate(date, undefined, options)
        ?.toString()
        .slice(0, 24)
    ).toBe('Wed Feb 07 2024 00:00:00');
  });
});

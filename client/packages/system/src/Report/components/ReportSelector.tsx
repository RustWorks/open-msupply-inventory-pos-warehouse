import React, { FC, PropsWithChildren } from 'react';
import { Box, ReportContext, Typography } from '@openmsupply-client/common';
import { AlertIcon } from '@common/icons';
import { useTranslation } from '@common/intl';
import {
  CircularProgress,
  FlatButton,
  PaperPopoverSection,
  usePaperClickPopover,
} from '@common/components';
import { ReportRowFragment, useReport } from '../api';

interface ReportSelectorProps {
  context?: ReportContext;
  onPrint: (report: ReportRowFragment) => void;
  disabled?: boolean;
}

const NoReports = ({ hasPermission }: { hasPermission: boolean }) => {
  const t = useTranslation('common');
  return (
    <Box display="flex" alignItems="center" gap={1} padding={2}>
      <Box flex={0}>
        <AlertIcon fontSize="small" color="primary" />
      </Box>
      <Typography flex={1}>
        {t(
          hasPermission
            ? 'error.no-reports-available'
            : 'error.no-report-permission'
        )}
      </Typography>
    </Box>
  );
};

export const ReportSelector: FC<PropsWithChildren<ReportSelectorProps>> = ({
  context,
  children,
  onPrint,
  disabled,
}) => {
  const { hide, PaperClickPopover } = usePaperClickPopover();
  const { data, isLoading } = useReport.document.list(context);
  const t = useTranslation('app');

  const reportButtons = data?.nodes?.map(report => (
    <FlatButton
      label={report.name}
      onClick={() => {
        hide();
        onPrint(report);
      }}
      key={report.id}
      sx={{ textAlign: 'left', justifyContent: 'left' }}
    />
  ));

  const hasPermission = !isLoading && data !== undefined;
  const noReports = !isLoading && !data?.nodes?.length;
  const oneReport =
    !isLoading && data?.nodes?.length === 1 ? data.nodes[0] : null;

  return !!oneReport && !disabled ? (
    <div onClick={() => onPrint(oneReport)}>{children}</div>
  ) : (
    <PaperClickPopover
      placement="bottom"
      width={350}
      Content={
        <PaperPopoverSection
          label={t('select-report')}
          labelStyle={{ width: '100%' }}
          alignItems="center"
        >
          {isLoading ? (
            <CircularProgress size={12} />
          ) : (
            <Box
              style={{
                maxHeight: '200px',
                overflowY: 'auto',
              }}
              display="flex"
              flexDirection="column"
            >
              {noReports || disabled ? (
                <NoReports hasPermission={hasPermission} />
              ) : (
                reportButtons
              )}
            </Box>
          )}
        </PaperPopoverSection>
      }
    >
      {children}
    </PaperClickPopover>
  );
};

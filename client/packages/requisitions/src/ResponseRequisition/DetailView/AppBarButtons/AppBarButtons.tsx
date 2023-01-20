import React from 'react';
import {
  AppBarButtonsPortal,
  Grid,
  LoadingButton,
  PrinterIcon,
  ReportContext,
  useDetailPanel,
  useTranslation,
} from '@openmsupply-client/common';
import {
  ReportRowFragment,
  ReportSelector,
  useReport,
} from '@openmsupply-client/system';
import { CreateShipmentButton } from './CreateShipmentButton';
import { SupplyRequestedQuantityButton } from './SupplyRequestedQuantityButton';
import { useResponse } from '../../api';
import { JsonData } from '@openmsupply-client/programs';

export const AppBarButtonsComponent = () => {
  const { OpenButton } = useDetailPanel();
  const { data } = useResponse.document.get();
  const { print, isPrinting } = useReport.utils.print();
  const t = useTranslation('common');

  const printReport = (
    report: ReportRowFragment,
    args: JsonData | undefined
  ) => {
    if (!data) return;
    print({ reportId: report.id, dataId: data?.id, args });
  };

  return (
    <AppBarButtonsPortal>
      <Grid container gap={1}>
        <CreateShipmentButton />
        <SupplyRequestedQuantityButton />
        <ReportSelector
          context={ReportContext.Requisition}
          onClick={printReport}
        >
          <LoadingButton
            variant="outlined"
            startIcon={<PrinterIcon />}
            isLoading={isPrinting}
          >
            {t('button.print')}
          </LoadingButton>
        </ReportSelector>
        {OpenButton}
      </Grid>
    </AppBarButtonsPortal>
  );
};

export const AppBarButtons = React.memo(AppBarButtonsComponent);

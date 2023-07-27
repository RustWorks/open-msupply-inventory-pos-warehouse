import React, { FC, useCallback } from 'react';
import {
  TableProvider,
  createTableStore,
  useEditModal,
  DetailViewSkeleton,
  AlertModal,
  useNavigate,
  RouteBuilder,
  useTranslation,
  createQueryParamsStore,
  DetailTabs,
  ModalMode,
} from '@openmsupply-client/common';
import { toItemRow, LogList } from '@openmsupply-client/system';
import { AppRoute } from '@openmsupply-client/config';
import { usePrescriptionIsDisabled } from '../api/hooks/utils/usePrescriptionIsDisabled';
import { usePrescription } from '../api/hooks';
import { ContentArea } from './ContentArea';
import { PrescriptionLineFragment } from '../api';
import { AppBarButtons } from './AppBarButton';
import { Toolbar } from './Toolbar';
import { SidePanel } from './SidePanel';
import { PrescriptionItem } from '../../types';
import { Draft } from '../..';
import { Footer } from './Footer';

export const PrescriptionDetailView: FC = () => {
  const isDisabled = usePrescriptionIsDisabled();
  const { onOpen, setMode } = useEditModal<Draft>();
  const { data, isLoading } = usePrescription.document.get();
  const t = useTranslation('dispensary');
  const navigate = useNavigate();
  const onRowClick = useCallback(
    (item: PrescriptionLineFragment | PrescriptionItem) => {
      onOpen({ item: toItemRow(item) });
    },
    [toItemRow, onOpen]
  );
  const onAddItem = (draft?: Draft) => {
    onOpen(draft);
    setMode(ModalMode.Create);
  };

  if (isLoading) return <DetailViewSkeleton hasGroupBy={true} hasHold={true} />;

  const tabs = [
    {
      Component: (
        <ContentArea
          onRowClick={!isDisabled ? onRowClick : null}
          onAddItem={onAddItem}
        />
      ),
      value: 'Details',
    },
    {
      Component: <LogList recordId={data?.id ?? ''} />,
      value: 'Log',
    },
  ];

  return (
    <React.Suspense
      fallback={<DetailViewSkeleton hasGroupBy={true} hasHold={true} />}
    >
      {data ? (
        <TableProvider
          createStore={createTableStore}
          queryParamsStore={createQueryParamsStore<
            PrescriptionLineFragment | PrescriptionItem
          >({
            initialSortBy: {
              key: 'itemName',
            },
          })}
        >
          <AppBarButtons onAddItem={onAddItem} />

          <Toolbar />
          <DetailTabs tabs={tabs} />
          <Footer />
          <SidePanel />
        </TableProvider>
      ) : (
        <AlertModal
          open={true}
          onOk={() =>
            navigate(
              RouteBuilder.create(AppRoute.Dispensary)
                .addPart(AppRoute.Prescription)
                .build()
            )
          }
          title={t('error.prescription-not-found')}
          message={t('messages.click-to-return-to-prescriptions')}
        />
      )}
    </React.Suspense>
  );
};

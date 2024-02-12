import React, { useCallback } from 'react';
import {
  TableProvider,
  createTableStore,
  useEditModal,
  DetailViewSkeleton,
  AlertModal,
  RouteBuilder,
  useNavigate,
  useTranslation,
  DetailTabs,
  useRowHighlight,
  Column,
  SortBy,
} from '@openmsupply-client/common';
import { ItemRowFragment, ActivityLogList } from '@openmsupply-client/system';
import { Toolbar } from './Toolbar';
import { Footer } from './Footer';
import { AppBarButtons } from './AppBarButtons';
import { SidePanel } from './SidePanel';
import { StocktakeSummaryItem } from '../../types';

import { StocktakeLineEdit } from './modal/StocktakeLineEdit';
import { ContentArea } from './ContentArea';
import { AppRoute } from '@openmsupply-client/config';
import { StocktakeLineFragment, useStocktake } from '../api';
import { StocktakeLineErrorProvider } from '../context';

const StocktakeTabs = ({
  id,
  onOpen,
  // rows,
  onChangeSortBy,
  sortBy,
  items,
  lines,
  setItemFilter,
  itemFilter,
}: {
  id: string | undefined;
  onChangeSortBy: (
    column: Column<StocktakeLineFragment | StocktakeSummaryItem>
  ) => void;
  onOpen: (item?: ItemRowFragment | null | undefined) => void;
  // rows: StocktakeLineFragment[] | StocktakeSummaryItem[] | undefined;
  sortBy: SortBy<StocktakeLineFragment | StocktakeSummaryItem>;
  itemFilter: string;
  items: StocktakeSummaryItem[];
  lines: StocktakeLineFragment[];
  setItemFilter: (filter: string) => void;
}) => {
  const isDisabled = useStocktake.utils.isDisabled();

  const onRowClick = useCallback(
    (item: StocktakeLineFragment | StocktakeSummaryItem) => {
      if (item.item) onOpen(item.item);
    },
    [onOpen]
  );

  const tabs = [
    {
      Component: (
        <ContentArea
          onRowClick={!isDisabled ? onRowClick : null}
          onAddItem={() => onOpen()}
          // rows={rows}
          onChangeSortBy={onChangeSortBy}
          sortBy={sortBy}
          items={items}
          lines={lines}
          itemFilter={itemFilter}
          setItemFilter={setItemFilter}
        />
      ),
      value: 'Details',
    },
    {
      Component: <ActivityLogList recordId={id ?? ''} />,
      value: 'Log',
    },
  ];
  return (
    <TableProvider createStore={createTableStore}>
      <Toolbar
        items={items}
        lines={lines}
        itemFilter={itemFilter}
        setItemFilter={setItemFilter}
      />
      <DetailTabs tabs={tabs} />
    </TableProvider>
  );
};
let now = new Date().getTime();
export const DetailView = () => {
  const { isOpen, entity, onOpen, onClose, mode } =
    useEditModal<ItemRowFragment>();
  const { data: stocktake, isLoading } = useStocktake.document.get();
  console.log(
    `== DETAIL VIEW loading: ${isLoading} ${new Date().getTime() - now} ==`
  );
  // now = new Date().getTime();
  const { itemFilter, items, lines, setItemFilter, sortBy, onChangeSortBy } =
    useStocktake.line.rows(stocktake);
  const navigate = useNavigate();
  const t = useTranslation('inventory');
  const { HighlightStyles } = useRowHighlight();

  if (isLoading) return <DetailViewSkeleton hasGroupBy={true} hasHold={true} />;

  return !!lines ? (
    <StocktakeLineErrorProvider>
      <HighlightStyles />
      <AppBarButtons onAddItem={() => onOpen()} stocktake={stocktake} />

      <Footer stocktake={stocktake} />
      <SidePanel stocktake={stocktake} />

      <StocktakeTabs
        id={stocktake?.id}
        onOpen={onOpen}
        sortBy={sortBy}
        onChangeSortBy={onChangeSortBy}
        items={items}
        lines={lines}
        itemFilter={itemFilter}
        setItemFilter={setItemFilter}
      />

      {isOpen && (
        <StocktakeLineEdit
          isOpen={isOpen}
          onClose={onClose}
          mode={mode}
          item={entity}
          items={items}
          lines={lines.filter(line => line.item.id === entity?.id)}
        />
      )}
    </StocktakeLineErrorProvider>
  ) : (
    <AlertModal
      open={true}
      onOk={() =>
        navigate(
          RouteBuilder.create(AppRoute.Inventory)
            .addPart(AppRoute.Stocktakes)
            .build()
        )
      }
      title={t('error.stocktake-not-found')}
      message={t('messages.click-to-return')}
    />
  );
};

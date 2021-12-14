import React, { FC } from 'react';
import { Routes, Route } from 'react-router-dom';

import { RouteBuilder } from '@openmsupply-client/common';
import { AppRoute } from '@openmsupply-client/config';
import { ItemListView } from '../ListView';

const Service: FC = () => {
  const itemsRoute = RouteBuilder.create(AppRoute.Items).build();

  const itemRoute = RouteBuilder.create(AppRoute.OutboundShipment)
    .addPart(':id')
    .build();

  return (
    <Routes>
      <Route path={itemsRoute} element={<ItemListView />} />
      <Route path={itemRoute} element={<ItemListView />} />
    </Routes>
  );
};

export default Service;

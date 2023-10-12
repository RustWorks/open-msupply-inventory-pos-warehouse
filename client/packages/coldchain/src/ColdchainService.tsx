import React, { FC } from 'react';
import { RouteBuilder, Routes, Route } from '@openmsupply-client/common';
import { AppRoute } from '@openmsupply-client/config';
import { ListView } from './Sensor/ListView';

export const ColdchainService: FC = () => {
  const sensorRoute = RouteBuilder.create(AppRoute.Sensors).build();

  return (
    <Routes>
      <Route path={sensorRoute} element={<ListView />} />
    </Routes>
  );
};

export default ColdchainService;

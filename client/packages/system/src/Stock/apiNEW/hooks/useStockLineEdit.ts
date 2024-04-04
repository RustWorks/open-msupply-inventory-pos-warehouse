import {
  ObjUtils,
  RecordPatch,
  setNullableInput,
  useMutation,
} from '@openmsupply-client/common';
import { StockLineRowFragment } from '../../api/operations.generated';
import { useGraphQL } from '../useGraphQL';
import { useState } from 'react';

export const useStockLineEdit = (init: StockLineRowFragment) => {
  const { stockApi, storeId, queryClient } = useGraphQL();
  const [stockLine, setStockLine] = useState<StockLineRowFragment>({ ...init });

  const updateDraft = (patch: Partial<StockLineRowFragment>) => {
    const newStockLine = { ...stockLine, ...patch };
    if (ObjUtils.isEqual(stockLine, newStockLine)) return;
    setStockLine(newStockLine);
  };

  const mutationFn = async (patch: RecordPatch<StockLineRowFragment>) => {
    const result =
      (await stockApi.updateStockLine({
        storeId,
        input: {
          id: patch?.id,
          location: setNullableInput('id', patch.location),
          costPricePerPack: patch.costPricePerPack,
          sellPricePerPack: patch.sellPricePerPack,
          expiryDate: patch.expiryDate,
          batch: patch.batch,
          onHold: patch.onHold,
          barcode: patch.barcode,
        },
      })) || {};

    const { updateStockLine } = result;

    if (updateStockLine?.__typename === 'StockLineNode') {
      return patch;
    }

    throw new Error('Unable to update stock line');
  };

  const mutation = useMutation({
    mutationFn,
    onSuccess: () => queryClient.invalidateQueries(['stock']),
  });

  return {
    draft: stockLine,
    updateDraft,
    saveStockLine: () => mutation.mutateAsync(stockLine),
  };
};

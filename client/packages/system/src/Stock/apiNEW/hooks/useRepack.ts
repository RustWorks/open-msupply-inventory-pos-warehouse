import { useMutation, useQuery } from '@openmsupply-client/common';
import { useGraphQL } from '../useGraphQL';
import { useState } from 'react';
import { Repack } from '../../types';

export const useRepack = (invoiceId: string) => {
  const { stockApi, storeId } = useGraphQL();

  const queryFn = async () => {
    const result = await stockApi.repack({
      storeId,
      invoiceId,
    });

    if (result.repack.__typename === 'RepackNode') {
      return result.repack;
    }
  };

  const query = useQuery({
    queryKey: ['stock', invoiceId],
    queryFn,
    enabled: invoiceId !== '',
  });

  return query;
};

export const useRepackEdit = (init: Repack) => {
  const { stockApi, storeId, queryClient } = useGraphQL();
  const [repack, setRepack] = useState<Repack>({ ...init });
  const stockLineId = repack.stockLineId ?? '';

  // FETCH
  const queryFn = async () => {
    const result = await stockApi.repacksByStockLine({
      storeId,
      stockLineId,
    });

    return result.repacksByStockLine;
  };

  const { data, isError, isLoading } = useQuery({
    queryKey: ['stock', storeId, stockLineId],
    queryFn,
    enabled: stockLineId !== '',
  });

  // UPDATE
  const onChange = (patch: Partial<Repack>) => {
    setRepack({ ...repack, ...patch });
  };

  const mutationFn = async () => {
    const result = await stockApi.insertRepack({
      storeId,
      input: {
        stockLineId: repack.stockLineId ?? '',
        newPackSize: repack.newPackSize ?? 0,
        numberOfPacks: repack.numberOfPacks ?? 0,
        newLocationId: repack.newLocationId ?? undefined,
      },
    });

    return result.insertRepack;
  };

  const mutation = useMutation({
    mutationFn,
    onSuccess: () => {
      // Stock list needs to be re-fetched to load new repacked stock line
      queryClient.invalidateQueries(['stock', storeId, 'list']);
      // Repack list also needs to be re-fetched on insert to show new repack
      // line
      queryClient.invalidateQueries(['stock', storeId, repack.stockLineId]);
    },
  });

  return {
    // Fetch
    repacks: data ? data?.nodes : undefined,
    isError,
    isLoading,
    // Update
    draft: repack,
    onChange,
    onInsert: mutation.mutateAsync,
  };
};

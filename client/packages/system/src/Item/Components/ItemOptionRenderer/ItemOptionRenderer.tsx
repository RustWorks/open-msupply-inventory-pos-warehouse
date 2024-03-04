import React from 'react';
import { ItemOption } from '../../utils';
import { Tooltip } from '@common/components';
import { ItemStockOnHandFragment } from '../../api';
import { usePackVariant } from '../../context/usePackVariant';

export const getItemOptionRenderer =
  () =>
  (
    props: React.HTMLAttributes<HTMLLIElement>,
    item: ItemStockOnHandFragment
  ) => (
    <Tooltip title={`${item.code} ${item.name}`}>
      <ItemOption {...props} key={item.code}>
        <span
          style={{
            whiteSpace: 'nowrap',
            width: 150,
            overflow: 'hidden',
            textOverflow: 'ellipsis',
          }}
        >
          {item.code}
        </span>
        <span style={{ whiteSpace: 'normal', width: 500 }}>{item.name}</span>
        <PackVariantAvailableQuantityDisplay item={item} />
      </ItemOption>
    </Tooltip>
  );

const PackVariantAvailableQuantityDisplay = ({
  item,
}: {
  item: ItemStockOnHandFragment;
}) => {
  const { activePackVariant, numberOfPacksFromQuantity } = usePackVariant(
    item?.id ?? '',
    item?.unitName ?? null
  );

  return (
    <span
      style={{
        width: 200,
        textAlign: 'right',
      }}
    >{`${numberOfPacksFromQuantity(
      item.availableStockOnHand
    )} (${activePackVariant})`}</span>
  );
};

import { usePackUnits } from '../api';
import { create } from 'zustand';
import { ItemPackUnitNode, UnitNode } from '@common/types';
import { ArrayUtils, NumUtils, isEqual } from '@common/utils';
import { useEffect } from 'react';
import { LocaleKey, TypedTFunction, useTranslation } from '@common/intl';
import { useAuthContext, useLocalStorage } from '@openmsupply-client/common';

interface UnitState {
  // From back end
  items: {
    [itemId: string]: ItemPackUnitNode;
  };
  // Should be called on startup when fetching multi unit variants
  setItems: (newItems: ItemPackUnitNode[]) => void;
}

const useUnitStore = create<UnitState>(set => {
  return {
    items: {},
    setItems: newItems =>
      set(() => {
        return {
          // Using function for iterator instead of just itemId for type safety
          items: ArrayUtils.keyBy(newItems, item => item.itemId),
        };
      }),
  };
});

export const useInitUnitStore = () => {
  const { setItems } = useUnitStore();
  // This should happen on startup and when store is changed (when the store changed, calculated values of mostUsedVariant and app data userSelected would change)
  // Suggested places:
  // https://github.com/openmsupply/open-msupply/blob/312b837c3d17a1ead05e140b7668cd5f45dffbc3/client/packages/common/src/authentication/api/hooks/useLogin.ts#L107
  // https://github.com/openmsupply/open-msupply/blob/312b837c3d17a1ead05e140b7668cd5f45dffbc3/client/packages/common/src/authentication/AuthContext.tsx#L125
  const { data } = usePackUnits();

  useEffect(() => {
    if (!data) return;
    setItems(data.nodes || []);
  }, [data]);

  // TODO add user selected from app data
};

type CommonAsPackUnit = (_: {
  packSize: number;
  packUnitName?: string;
  unitName: string | null;
  defaultPackUnit?: string;
  t: TypedTFunction<LocaleKey>;
}) => string;
const commonAsPackUnit: CommonAsPackUnit = ({
  packSize,
  packUnitName,
  unitName,
  defaultPackUnit,
  t,
}) => {
  if (packUnitName) return packUnitName;
  if (defaultPackUnit) return defaultPackUnit;
  if (unitName) return `${packSize} ${unitName}`;

  const defaultUnit = t('label.unit');
  return `${packSize} ${defaultUnit}`;
};

export interface VariantControl {
  variants: UnitNode[];
  // Selected by user or mostUsed (calculated by backend)
  activeVariant: UnitNode;
  setUserSelectedVariant: (variantId: string) => void;
}

export const useUnitVariant = (
  itemId: string,
  unitName: string | null
): {
  // If pack unit variant not found, use defaultPackUnit rathern then
  // {packSize} {unitName or 'Unit'}
  asPackUnit: (packSize: number, defaultPackUnit?: string) => string;
  activePackUnit: string;
  numberOfPacksFromQuantity: (totalQuantity: number) => number;
  numberOfPacksToTotalQuantity: (numPacks: number) => number;
  variantsControl?: VariantControl;
  unitVariantsExist: boolean;
} => {
  const authContext = useAuthContext();
  const [userSelectedVariants, setUserSelectedVariant] = useLocalStorage(
    `/user/${authContext.user?.id ?? '?'}/store/${
      authContext.storeId
    }/selectedVariants`
  );
  const userSelectedVariantId = userSelectedVariants?.[itemId];
  const [item] = useUnitStore(state => [state.items[itemId]], isEqual);
  const t = useTranslation();

  if (!item || item.packUnits.length == 0) {
    return {
      asPackUnit: (packSize, defaultPackUnit) =>
        commonAsPackUnit({ packSize, unitName, t, defaultPackUnit }),
      numberOfPacksFromQuantity: totalQuantity => totalQuantity,
      numberOfPacksToTotalQuantity: numPacks => numPacks,
      unitVariantsExist: false,
      activePackUnit: commonAsPackUnit({ packSize: 1, unitName, t }),
    };
  }

  const { packUnits, mostUsedPackUnitId } = item;

  const mostUsedVariant = packUnits.find(({ id }) => id === mostUsedPackUnitId);
  const userSelectedVariant = packUnits.find(
    ({ id }) => id === userSelectedVariantId
  );

  const activeVariant =
    userSelectedVariant ||
    mostUsedVariant ||
    (packUnits[0] as UnitNode); /* item.variants.length === 0 above confirms that it's safe to assume it will not be undefined */

  return {
    asPackUnit: (packSize, defaultPackUnit) => {
      const foundVariant = packUnits.find(
        packUnits => packUnits.packSize === packSize
      );

      return commonAsPackUnit({
        packSize,
        unitName,
        packUnitName: foundVariant?.shortName,
        defaultPackUnit,
        t,
      });
    },
    numberOfPacksFromQuantity: totalQuantity =>
      NumUtils.round(totalQuantity / activeVariant.packSize, 2),
    numberOfPacksToTotalQuantity: numPacks =>
      NumUtils.round(numPacks * activeVariant.packSize, 2),
    // TODO what if variants were soft deleted ?
    variantsControl: {
      variants: packUnits,
      activeVariant,
      setUserSelectedVariant: variantId =>
        setUserSelectedVariant({
          ...userSelectedVariants,
          [itemId]: variantId,
        }),
    },
    unitVariantsExist: true,
    activePackUnit: commonAsPackUnit({
      packSize: activeVariant.packSize,
      unitName,
      t,
    }),
  };
};
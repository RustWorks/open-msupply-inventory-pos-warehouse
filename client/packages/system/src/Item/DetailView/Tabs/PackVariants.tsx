import React, { FC } from 'react';
import { VariantFragment } from '../../api';
import {
  Box,
  DataTable,
  TableProvider,
  useColumns,
  createTableStore,
  NothingHere,
  useTranslation,
  ButtonWithIcon,
  PlusCircleIcon,
  Grid,
  useEditModal,
  useAuthContext,
  useNotification,
  UserPermission,
  DropdownMenu,
  DropdownMenuItem,
  DeleteIcon,
} from '@openmsupply-client/common';
import { usePackVariant } from '../../context';
import { PackVariantEditModal } from '../../Components/PackVariantEditModal';
import { usePackVariantDeleteSelected } from '../../api/hooks/usePackVariantDelete';

const PackVariantTable: FC<{ itemId: string }> = ({ itemId }) => {
  const t = useTranslation('catalogue');
  const { variantsControl } = usePackVariant(itemId, null);
  const { isOpen, entity, mode, onClose, onOpen } =
    useEditModal<VariantFragment>();
  const { info } = useNotification();
  const { userHasPermission } = useAuthContext();
  const infoSnack = info(t('auth.permission-denied'));
  const hasPermission = userHasPermission(
    UserPermission.ItemNamesCodesAndUnitsMutate
  );
  const variants = variantsControl?.variants ?? [];
  const onDelete = usePackVariantDeleteSelected(variants);

  const columns = useColumns<VariantFragment>([
    'packSize',
    {
      key: 'shortName',
      label: 'label.abbreviation',
      accessor: ({ rowData }) => rowData?.shortName,
      sortable: false,
    },
    {
      key: 'longName',
      label: 'label.name',
      accessor: ({ rowData }) => rowData?.longName,
      sortable: false,
    },
    'selection',
  ]);

  return (
    <>
      {isOpen && (
        <PackVariantEditModal
          mode={mode}
          isOpen={isOpen}
          onClose={onClose}
          packVariant={entity}
          itemId={itemId ?? ''}
        />
      )}
      <Box display="flex" justifyContent="flex-end" paddingBottom={2}>
        <ButtonWithIcon
          Icon={<PlusCircleIcon />}
          label={t('label.new-pack-variant')}
          onClick={hasPermission ? () => onOpen() : () => infoSnack()}
        />
      </Box>
      <Box display="flex" justifyContent="flex-end" paddingBottom={2}>
        <DropdownMenu label="Select">
          <DropdownMenuItem IconComponent={DeleteIcon} onClick={onDelete}>
            {t('button.delete-lines')}
          </DropdownMenuItem>
        </DropdownMenu>
      </Box>
      <DataTable
        id="item-variants-detail"
        data={variants}
        columns={columns}
        noDataElement={<NothingHere body={t('error.no-pack-variants')} />}
        onRowClick={hasPermission ? onOpen : infoSnack}
      />
    </>
  );
};

interface PackVariantsTabProps {
  itemId: string;
}

export const PackVariantsTab: FC<PackVariantsTabProps> = ({ itemId }) => {
  return (
    <Box justifyContent="center" display="flex" flex={1} paddingTop={3}>
      <Grid container gap={1} maxWidth={1000} display="block">
        <TableProvider createStore={createTableStore}>
          <PackVariantTable itemId={itemId} />
        </TableProvider>
      </Grid>
    </Box>
  );
};

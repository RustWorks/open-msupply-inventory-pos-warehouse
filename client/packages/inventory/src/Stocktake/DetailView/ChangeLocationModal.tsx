import React, { useState } from 'react';
import {
  InputWithLabelRow,
  useTranslation,
  ConfirmationModalLayout,
  DialogButton,
} from '@openmsupply-client/common';
import {
  LocationRowFragment,
  LocationSearchInput,
} from '@openmsupply-client/system';
import { useStocktake } from '../api';

interface ChangeLocationConfirmationModalProps {
  isOpen: boolean;
  onCancel: () => void;
}

export const ChangeLocationConfirmationModal = ({
  isOpen,
  onCancel,
}: ChangeLocationConfirmationModalProps) => {
  const t = useTranslation('inventory');

  const [location, setLocation] = useState<LocationRowFragment | null>(null);

  const onChangeLocation = useStocktake.line.changeLocation();

  return (
    <ConfirmationModalLayout
      isOpen={isOpen}
      title={t('heading.are-you-sure')}
      message={t('messages.confirm-change-location')}
      buttons={[
        <DialogButton variant="cancel" onClick={onCancel} />,
        <DialogButton
          variant="ok"
          onClick={async () => {
            await onChangeLocation(location);
            onCancel();
          }}
        />,
      ]}
    >
      <InputWithLabelRow
        label={t('label.location')}
        labelWidth="100px"
        Input={
          <LocationSearchInput
            disabled={false}
            selectedLocation={location}
            onChange={setLocation}
            width={200}
          />
        }
      />
    </ConfirmationModalLayout>
  );
};

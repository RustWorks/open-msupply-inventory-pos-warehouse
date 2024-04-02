import React from 'react';
import {
  ButtonWithIcon,
  useTranslation,
  useToggle,
  PlusCircleIcon,
  InvoiceNodeStatus,
  useAuthContext,
} from '@openmsupply-client/common';
import { MasterListSearchModal } from '@openmsupply-client/system';
import { useInbound } from '../api';

type Result<R, E> = { type: 'ok'; data: R } | { type: 'error'; error: E };
type OperationState<R, E> = 'loading' | Result<R, E>;

type CommonFetchErrors<E> = 'some network thing' | 'some axios thing' | E;
type UseApiCall<
  R extends object,
  FE,
  /* Original Fetch Error */ SME /* Save Mutation Error */,
  Input,
> = (input: Input) => {
  update: (patch: Partial<R>) => void;

  save: () => Promise<Result<R, CommonFetchErrors<SME>>>;
  saveState: OperationState<R, CommonFetchErrors<SME>>;

  updateAndSave: (
    patch: Partial<R>
  ) => Promise<Result<R, CommonFetchErrors<SME>>>;
  udpateAndSaveState: () => OperationState<R, CommonFetchErrors<SME>>;

  state: OperationState<R, CommonFetchErrors<FE>>;
};

// const useYowsit: useApiCall<> = ()

type Yow = {
  id: string;
  isHowsit?: boolean;
  up2: string;
  somethingThatIsUpdatedOnTheGo: string;
};
type FetchError = 'does not exit';
type SaveError = 'cant have up2 without howsit';
// Could just be type extractor on hook, ie SaveResult<typeof useYowsit>
type SaveResult = Result<Yow, CommonFetchErrors<SaveError>>;
type UseYowsit = UseApiCall<Yow, FetchError, SaveError, String>;

const useYowsit: UseYowsit = id => {
  /* TODO */
};

const Yowsit = () => {
  const { update, save, saveState, updateAndSave, state } = useYowsit('yowid');

  const handleSaveResult = (result: SaveResult) => {
    if (result.type == 'error') {
      /* handle it */
      return null;
    }
  };

  /* more explicit loading state check */
  if (state === 'loading') {
    /* display spinner */
    return null;
  }

  if (state.type === 'error') {
    /* handle errors */
    return null;
  }

  const yow = state.data;

  return (
    <>
      <Checkbox
        onChange={e => update({ isHowsit: e.target.checked })}
        checked={yow.isHowsit}
      />
      <Input
        type="text"
        onChange={e => update({ up2: e.target.value })}
        value={yow.up2}
      />
      <Input
        type="text"
        onChange={e => async () => {
          const result = await updateAndSave({
            somethingThatIsUpdatedOnTheGo: e.target.value,
          });
          handleSaveResult(result);
        }}
        value={/* this value is debounced */ yow.somethingThatIsUpdatedOnTheGo}
      />
      <LoadingButton
        title={'Save'}
        onClick={async () => handleSaveResult(await save())}
        isLoading={saveState == 'loading'}
      />
    </>
  );
};

export const AddFromMasterListButtonComponent = () => {
  const t = useTranslation('distribution');
  const { addFromMasterList } = useInbound.utils.addFromMasterList();
  const { storeId } = useAuthContext();
  const { status } = useInbound.document.fields(['status']);
  const isDisabled = status !== InvoiceNodeStatus.New;
  const modalController = useToggle();
  const filterByStore = { existsForStoreId: { equalTo: storeId } };

  return (
    <>
      <MasterListSearchModal
        open={modalController.isOn}
        onClose={modalController.toggleOff}
        onChange={masterList => {
          modalController.toggleOff();
          addFromMasterList(masterList);
        }}
        filterBy={filterByStore}
      />
      <ButtonWithIcon
        disabled={isDisabled}
        Icon={<PlusCircleIcon />}
        label={t('button.add-from-master-list')}
        onClick={modalController.toggleOn}
      />
    </>
  );
};

export const AddFromMasterListButton = React.memo(
  AddFromMasterListButtonComponent
);

import React, { useMemo, useState, useEffect } from 'react';
import {
  ArrowRightIcon,
  useTranslation,
  useNotification,
  SplitButtonOption,
  useConfirmationModal,
  RequisitionNodeStatus,
  ButtonWithIcon,
  useAuthContext,
  UserPermission,
  Tooltip,
} from '@openmsupply-client/common';
import { getNextRequestStatus, getStatusTranslation } from '../../../utils';
import { useRequest } from '../../api';

const getStatusOptions = (
  currentStatus: RequisitionNodeStatus,
  getButtonLabel: (status: RequisitionNodeStatus) => string
): SplitButtonOption<RequisitionNodeStatus>[] => {
  const options: [
    SplitButtonOption<RequisitionNodeStatus>,
    SplitButtonOption<RequisitionNodeStatus>,
    SplitButtonOption<RequisitionNodeStatus>
  ] = [
    {
      value: RequisitionNodeStatus.Draft,
      label: getButtonLabel(RequisitionNodeStatus.Draft),
      isDisabled: true,
    },
    {
      value: RequisitionNodeStatus.Sent,
      label: getButtonLabel(RequisitionNodeStatus.Sent),
      isDisabled: true,
    },
    {
      value: RequisitionNodeStatus.Finalised,
      label: getButtonLabel(RequisitionNodeStatus.Finalised),
      isDisabled: true,
    },
  ];

  if (currentStatus === RequisitionNodeStatus.Draft) {
    options[1].isDisabled = false;
  }

  return options;
};

const getNextStatusOption = (
  status: RequisitionNodeStatus,
  options: SplitButtonOption<RequisitionNodeStatus>[]
): SplitButtonOption<RequisitionNodeStatus> | null => {
  if (!status) return options[0] ?? null;

  const nextStatus = getNextRequestStatus(status);
  const nextStatusOption = options.find(o => o.value === nextStatus);
  return nextStatusOption || null;
};

const getButtonLabel =
  (t: ReturnType<typeof useTranslation>) =>
  (invoiceStatus: RequisitionNodeStatus): string => {
    return t('button.save-and-confirm-status', {
      status: t(getStatusTranslation(invoiceStatus)),
    });
  };

const useStatusChangeButton = () => {
  const { status, update, comment } = useRequest.document.fields([
    'status',
    'comment',
  ]);
  const { success, error } = useNotification();
  const t = useTranslation('replenishment');
  const { user } = useAuthContext();

  const options = useMemo(
    () => getStatusOptions(status, getButtonLabel(t)),
    [status, getButtonLabel]
  );

  const [selectedOption, setSelectedOption] =
    useState<SplitButtonOption<RequisitionNodeStatus> | null>(() =>
      getNextStatusOption(status, options)
    );

  const getUpdatedComment = () => {
    if (selectedOption?.value !== RequisitionNodeStatus.Sent) {
      return comment;
    }

    // TODO: change to `getLocalisedFullName` when the programs feature is merged
    const name = `${user?.firstName ?? ''} ${user?.lastName ?? ''}`.trim();
    const job = !!user?.jobTitle ? ` (${user?.jobTitle})` : '';

    return `${comment ? comment + '\n' : ''}${t('template.requisition-sent', {
      name: name || user?.name,
      job,
      phone: user?.phoneNumber ?? '-',
      email: user?.email ?? '-',
    })}`;
  };

  const onConfirmStatusChange = async () => {
    if (!selectedOption) return null;
    try {
      await update({
        status: selectedOption.value,
        comment: getUpdatedComment(),
      });
      success(t('messages.saved'))();
    } catch (e) {
      error(t('messages.could-not-save'))();
    }
  };

  const getConfirmation = useConfirmationModal({
    title: t('heading.are-you-sure'),
    message: t('messages.confirm-status-as', {
      status: selectedOption?.value
        ? getStatusTranslation(selectedOption?.value)
        : '',
    }),
    onConfirm: onConfirmStatusChange,
  });

  // When the status changes (after an update), set the selected option to the next status.
  // Otherwise, it would be set to the current status, which is now a disabled option.
  useEffect(() => {
    setSelectedOption(() => getNextStatusOption(status, options));
  }, [status, options]);

  return { options, selectedOption, setSelectedOption, getConfirmation };
};

export const StatusChangeButton = () => {
  const { selectedOption, getConfirmation } = useStatusChangeButton();
  const isDisabled = useRequest.utils.isDisabled();
  const { userHasPermission } = useAuthContext();
  const t = useTranslation('app');

  if (!selectedOption) return null;
  if (isDisabled) return null;

  const hasPermission =
    selectedOption.value === RequisitionNodeStatus.Sent
      ? userHasPermission(UserPermission.RequisitionSend)
      : true;

  return (
    <>
      <Tooltip title={hasPermission ? '' : t('auth.permission-denied')}>
        <div>
          <ButtonWithIcon
            color="secondary"
            variant="contained"
            disabled={!hasPermission}
            label={selectedOption.label}
            Icon={<ArrowRightIcon />}
            onClick={() => getConfirmation()}
          />
        </div>
      </Tooltip>
    </>
  );
};

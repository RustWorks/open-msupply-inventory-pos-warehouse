import React, { FC, useEffect, useState } from 'react';
import {
  useTranslation,
  DetailViewSkeleton,
  AlertModal,
  useNavigate,
  RouteBuilder,
  useDebounceCallback,
  useBreadcrumbs,
  useFormatDateTime,
  Breadcrumb,
  useIntlUtils,
} from '@openmsupply-client/common';
import {
  useJsonForms,
  useContactTraces,
  ContactTraceRowFragment,
  useDocumentDataAccessor,
  JsonFormData,
  SavedDocument,
} from '@openmsupply-client/programs';
import { AppRoute } from '@openmsupply-client/config';
import { Toolbar } from './Toolbar';
import { Footer } from './Footer';
import { PatientTabValue } from '../../Patient/PatientView/PatientView';

import { ContactTrace, useContactTraceData } from './useContactTraceData';

type DetailViewProps = {
  createPatientId: string | null;
  createType: string | null;
};

export const DetailView: FC<DetailViewProps> = ({
  createType,
  createPatientId,
}) => {
  const t = useTranslation('dispensary');
  const navigate = useNavigate();
  const { setSuffix } = useBreadcrumbs([AppRoute.ContactTrace]);
  const dateFormat = useFormatDateTime();
  const { getLocalisedFullName } = useIntlUtils();
  const id = useContactTraces.utils.idFromUrl();
  const { data: contactData, isLoading } = useContactTraceData(
    id,
    createType,
    createPatientId
  );
  // Used when creating a new contact trace, i.e. to check if it differs from the creation id from
  // the URL.
  const [contactTraceId, setContactTraceId] = useState(id);

  const handleSave = useContactTraces.document.upsertDocument(
    contactData?.patient?.id ?? '',
    contactData?.type ?? '',
    contactTrace => {
      if (contactTrace.id !== id) setContactTraceId(contactTrace.id);
    }
  );

  const dataAccessor = useDocumentDataAccessor(
    contactData?.documentName,
    undefined,
    handleSave
  );
  const accessor: JsonFormData<SavedDocument> =
    createType && contactData?.schema
      ? {
          loadedData: contactData.contactTrace,
          isLoading: false,
          error: undefined,
          isCreating: contactTraceId === id,
          schema: contactData?.schema,
          save: async (data: unknown) => {
            return await handleSave(
              data,
              contactData.schema?.formSchemaId ?? ''
            );
          },
        }
      : dataAccessor;

  const {
    JsonForm,
    data,
    setData,
    saveData,
    isDirty,
    validationError,
    revert,
  } = useJsonForms(
    {
      documentName: contactData?.documentName,
      patientId: contactData?.patient?.id,
    },
    accessor
  );

  // When a contact trace id changes (contact trace has been created), wait till the isDirty flag
  // is cleared and then navigate to the correct url.
  useEffect(() => {
    if (!isDirty && contactTraceId !== id) {
      navigate(
        RouteBuilder.create(AppRoute.Dispensary)
          .addPart(AppRoute.ContactTrace)
          .addPart(contactTraceId)
          .build()
      );
    }
  }, [isDirty, contactTraceId, id, navigate]);

  const updateContactTrace = useDebounceCallback(
    (patch: Partial<ContactTrace>) =>
      setData({
        ...(typeof data === 'object' ? data : {}),
        ...patch,
      }),
    [data, setData]
  );

  useEffect(() => {
    if (contactData) {
      setSuffix(
        <span key="patient-contact-trace">
          <Breadcrumb
            to={RouteBuilder.create(AppRoute.Dispensary)
              .addPart(AppRoute.Patients)
              .addPart(contactData.patient.id)
              .addQuery({ tab: PatientTabValue.ContactTracing })
              .build()}
          >
            {getLocalisedFullName(
              contactData.patient?.firstName,
              contactData.patient?.lastName
            )}
          </Breadcrumb>
          <span>{` / ${contactData.programName} - ${dateFormat.localisedDate(
            contactData.contactTrace.datetime
          )}`}</span>
        </span>
      );
    }
  }, [contactData, setSuffix]);

  if (isLoading) return <DetailViewSkeleton />;

  return (
    <React.Suspense fallback={<DetailViewSkeleton />}>
      <link rel="stylesheet" href="/medical-icons.css" media="all"></link>
      {contactData && (
        <Toolbar
          onChange={updateContactTrace}
          data={contactData}
          documentData={(data as ContactTrace) ?? contactData.contactTrace}
        />
      )}
      {!isLoading ? (
        JsonForm
      ) : (
        <AlertModal
          open={true}
          onOk={() =>
            navigate(
              RouteBuilder.create(AppRoute.Dispensary)
                .addPart(AppRoute.ContactTrace)
                .build()
            )
          }
          title={t('error.encounter-not-found')}
          message={t('messages.click-to-return-to-contact-traces')}
        />
      )}

      <Footer
        documentName={contactData?.documentName}
        onSave={saveData}
        onCancel={revert}
        isDisabled={!isDirty || !!validationError}
        contactTrace={data as ContactTraceRowFragment}
      />
    </React.Suspense>
  );
};

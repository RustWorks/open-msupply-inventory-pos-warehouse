import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type DocumentFragment = { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null };

export type DocumentByNameQueryVariables = Types.Exact<{
  name: Types.Scalars['String'];
  storeId: Types.Scalars['String'];
}>;


export type DocumentByNameQuery = { __typename: 'Queries', document?: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } | null };

export type DocumentsQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  page?: Types.InputMaybe<Types.PaginationInput>;
  filter?: Types.InputMaybe<Types.DocumentFilterInput>;
  sort?: Types.InputMaybe<Types.DocumentSortInput>;
}>;


export type DocumentsQuery = { __typename: 'Queries', documents: { __typename: 'DocumentConnector', nodes: Array<{ __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null }> } };

export type DocumentRegistryFragment = { __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any };

export type DocumentRegistriesQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.DocumentRegistryFilterInput>;
  sort?: Types.InputMaybe<Types.DocumentRegistrySortInput>;
}>;


export type DocumentRegistriesQuery = { __typename: 'Queries', documentRegistries: { __typename: 'DocumentRegistryConnector', totalCount: number, nodes: Array<{ __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any }> } };

export type DocumentRegistryWithChildrenFragment = { __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any, children: Array<{ __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any }> };

export type DocumentRegistriesWithChildrenQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.DocumentRegistryFilterInput>;
  sort?: Types.InputMaybe<Types.DocumentRegistrySortInput>;
}>;


export type DocumentRegistriesWithChildrenQuery = { __typename: 'Queries', documentRegistries: { __typename: 'DocumentRegistryConnector', totalCount: number, nodes: Array<{ __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any, children: Array<{ __typename: 'DocumentRegistryNode', id: string, documentType: string, context: Types.DocumentRegistryNodeContext, name?: string | null, parentId?: string | null, formSchemaId: string, jsonSchema: any, uiSchemaType: string, uiSchema: any }> }> } };

export type AllocateProgramNumberMutationVariables = Types.Exact<{
  numberName: Types.Scalars['String'];
  storeId: Types.Scalars['String'];
}>;


export type AllocateProgramNumberMutation = { __typename: 'Mutations', allocateProgramNumber: { __typename: 'NumberNode', number: number } };

export type EncounterFieldsFragment = { __typename: 'EncounterFieldsNode', fields: Array<any>, encounter: { __typename: 'EncounterNode', name: string, startDatetime: string, endDatetime?: string | null } };

export type EncounterFieldsQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  patientId: Types.Scalars['String'];
  fields?: Types.InputMaybe<Array<Types.Scalars['String']> | Types.Scalars['String']>;
}>;


export type EncounterFieldsQuery = { __typename: 'Queries', encounterFields: { __typename: 'EncounterFieldsConnector', nodes: Array<{ __typename: 'EncounterFieldsNode', fields: Array<any>, encounter: { __typename: 'EncounterNode', name: string, startDatetime: string, endDatetime?: string | null } }> } };

export type EncounterDocumentRegistryFragment = { __typename: 'DocumentRegistryNode', context: Types.DocumentRegistryNodeContext, documentType: string, formSchemaId: string, id: string, jsonSchema: any, name?: string | null, parentId?: string | null, uiSchema: any, uiSchemaType: string, children: Array<{ __typename: 'DocumentRegistryNode', id: string }> };

export type EncounterBaseFragment = { __typename: 'EncounterNode', id: string, type: string, name: string, status?: Types.EncounterNodeStatus | null, program: string, startDatetime: string, endDatetime?: string | null, patient: { __typename: 'NameNode', id: string, firstName?: string | null, lastName?: string | null, name: string }, document: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } };

export type EncounterByIdQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  encounterId: Types.Scalars['String'];
}>;


export type EncounterByIdQuery = { __typename: 'Queries', encounters: { __typename: 'EncounterConnector', totalCount: number, nodes: Array<{ __typename: 'EncounterNode', id: string, type: string, name: string, status?: Types.EncounterNodeStatus | null, program: string, startDatetime: string, endDatetime?: string | null, patient: { __typename: 'NameNode', id: string, firstName?: string | null, lastName?: string | null, name: string }, document: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } }> } };

export type EncountersQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  key?: Types.InputMaybe<Types.EncounterSortFieldInput>;
  desc?: Types.InputMaybe<Types.Scalars['Boolean']>;
  filter?: Types.InputMaybe<Types.EncounterFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
}>;


export type EncountersQuery = { __typename: 'Queries', encounters: { __typename: 'EncounterConnector', totalCount: number, nodes: Array<{ __typename: 'EncounterNode', id: string, type: string, name: string, status?: Types.EncounterNodeStatus | null, program: string, startDatetime: string, endDatetime?: string | null, patient: { __typename: 'NameNode', id: string, firstName?: string | null, lastName?: string | null, name: string }, document: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } }> } };

export type ProgramEventFragment = { __typename: 'ProgramEventNode', activeDatetime: string, type: string, data?: string | null };

export type ProgramEnrolmentRowFragment = { __typename: 'ProgramEnrolmentNode', enrolmentDatetime: string, name: string, patientId: string, programPatientId?: string | null, type: string, document: { __typename: 'DocumentNode', documentRegistry?: { __typename: 'DocumentRegistryNode', id: string, name?: string | null } | null }, events: Array<{ __typename: 'ProgramEventNode', activeDatetime: string, type: string, data?: string | null }> };

export type ProgramEnrolmentsQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  key: Types.ProgramEnrolmentSortFieldInput;
  desc?: Types.InputMaybe<Types.Scalars['Boolean']>;
  filter?: Types.InputMaybe<Types.ProgramEnrolmentFilterInput>;
  eventTime: Types.Scalars['String'];
}>;


export type ProgramEnrolmentsQuery = { __typename: 'Queries', programEnrolments: { __typename: 'ProgramEnrolmentConnector', totalCount: number, nodes: Array<{ __typename: 'ProgramEnrolmentNode', enrolmentDatetime: string, name: string, patientId: string, programPatientId?: string | null, type: string, document: { __typename: 'DocumentNode', documentRegistry?: { __typename: 'DocumentRegistryNode', id: string, name?: string | null } | null }, events: Array<{ __typename: 'ProgramEventNode', activeDatetime: string, type: string, data?: string | null }> }> } };

export const DocumentRegistryFragmentDoc = gql`
    fragment DocumentRegistry on DocumentRegistryNode {
  __typename
  id
  documentType
  context
  name
  parentId
  formSchemaId
  jsonSchema
  uiSchemaType
  uiSchema
}
    `;
export const DocumentRegistryWithChildrenFragmentDoc = gql`
    fragment DocumentRegistryWithChildren on DocumentRegistryNode {
  ...DocumentRegistry
  children {
    ...DocumentRegistry
  }
}
    ${DocumentRegistryFragmentDoc}`;
export const EncounterFieldsFragmentDoc = gql`
    fragment EncounterFields on EncounterFieldsNode {
  fields
  encounter {
    name
    startDatetime
    endDatetime
  }
}
    `;
export const EncounterDocumentRegistryFragmentDoc = gql`
    fragment EncounterDocumentRegistry on DocumentRegistryNode {
  context
  documentType
  formSchemaId
  id
  jsonSchema
  name
  parentId
  uiSchema
  uiSchemaType
  children {
    id
  }
}
    `;
export const DocumentFragmentDoc = gql`
    fragment Document on DocumentNode {
  id
  name
  parents
  author
  timestamp
  type
  data
  documentRegistry {
    uiSchemaType
    documentType
    context
    formSchemaId
    jsonSchema
    uiSchema
  }
}
    `;
export const EncounterBaseFragmentDoc = gql`
    fragment EncounterBase on EncounterNode {
  id
  type
  name
  status
  patient {
    id
    firstName
    lastName
    name
  }
  program
  startDatetime
  endDatetime
  document {
    ...Document
  }
}
    ${DocumentFragmentDoc}`;
export const ProgramEventFragmentDoc = gql`
    fragment ProgramEvent on ProgramEventNode {
  activeDatetime
  type
  data
}
    `;
export const ProgramEnrolmentRowFragmentDoc = gql`
    fragment ProgramEnrolmentRow on ProgramEnrolmentNode {
  enrolmentDatetime
  name
  patientId
  programPatientId
  type
  document {
    documentRegistry {
      id
      name
    }
  }
  events(at: $eventTime) {
    ...ProgramEvent
  }
}
    ${ProgramEventFragmentDoc}`;
export const DocumentByNameDocument = gql`
    query documentByName($name: String!, $storeId: String!) {
  document(name: $name, storeId: $storeId) {
    __typename
    ... on DocumentNode {
      ...Document
    }
  }
}
    ${DocumentFragmentDoc}`;
export const DocumentsDocument = gql`
    query documents($storeId: String!, $page: PaginationInput, $filter: DocumentFilterInput, $sort: DocumentSortInput) {
  documents(storeId: $storeId, page: $page, filter: $filter, sort: $sort) {
    __typename
    ... on DocumentConnector {
      nodes {
        ...Document
      }
    }
  }
}
    ${DocumentFragmentDoc}`;
export const DocumentRegistriesDocument = gql`
    query documentRegistries($filter: DocumentRegistryFilterInput, $sort: DocumentRegistrySortInput) {
  documentRegistries(filter: $filter, sort: $sort) {
    ... on DocumentRegistryConnector {
      __typename
      nodes {
        ...DocumentRegistry
      }
      totalCount
    }
  }
}
    ${DocumentRegistryFragmentDoc}`;
export const DocumentRegistriesWithChildrenDocument = gql`
    query documentRegistriesWithChildren($filter: DocumentRegistryFilterInput, $sort: DocumentRegistrySortInput) {
  documentRegistries(sort: $sort, filter: $filter) {
    ... on DocumentRegistryConnector {
      __typename
      totalCount
      nodes {
        __typename
        ...DocumentRegistryWithChildren
      }
    }
  }
}
    ${DocumentRegistryWithChildrenFragmentDoc}`;
export const AllocateProgramNumberDocument = gql`
    mutation allocateProgramNumber($numberName: String!, $storeId: String!) {
  allocateProgramNumber(input: {numberName: $numberName}, storeId: $storeId) {
    ... on NumberNode {
      __typename
      number
    }
  }
}
    `;
export const EncounterFieldsDocument = gql`
    query encounterFields($storeId: String!, $patientId: String!, $fields: [String!]) {
  encounterFields(
    input: {fields: $fields}
    storeId: $storeId
    sort: {key: startDatetime}
    filter: {patientId: {equalTo: $patientId}}
  ) {
    ... on EncounterFieldsConnector {
      __typename
      nodes {
        __typename
        ...EncounterFields
      }
    }
  }
}
    ${EncounterFieldsFragmentDoc}`;
export const EncounterByIdDocument = gql`
    query encounterById($storeId: String!, $encounterId: String!) {
  encounters(storeId: $storeId, filter: {id: {equalTo: $encounterId}}) {
    ... on EncounterConnector {
      __typename
      nodes {
        ...EncounterBase
      }
      totalCount
    }
  }
}
    ${EncounterBaseFragmentDoc}`;
export const EncountersDocument = gql`
    query encounters($storeId: String!, $key: EncounterSortFieldInput, $desc: Boolean, $filter: EncounterFilterInput, $page: PaginationInput) {
  encounters(
    storeId: $storeId
    sort: {key: $key, desc: $desc}
    filter: $filter
    page: $page
  ) {
    ... on EncounterConnector {
      __typename
      nodes {
        ...EncounterBase
      }
      totalCount
    }
  }
}
    ${EncounterBaseFragmentDoc}`;
export const ProgramEnrolmentsDocument = gql`
    query programEnrolments($storeId: String!, $key: ProgramEnrolmentSortFieldInput!, $desc: Boolean, $filter: ProgramEnrolmentFilterInput, $eventTime: String!) {
  programEnrolments(
    storeId: $storeId
    sort: {key: $key, desc: $desc}
    filter: $filter
  ) {
    ... on ProgramEnrolmentConnector {
      __typename
      nodes {
        ...ProgramEnrolmentRow
      }
      totalCount
    }
  }
}
    ${ProgramEnrolmentRowFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    documentByName(variables: DocumentByNameQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DocumentByNameQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<DocumentByNameQuery>(DocumentByNameDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'documentByName', 'query');
    },
    documents(variables: DocumentsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DocumentsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<DocumentsQuery>(DocumentsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'documents', 'query');
    },
    documentRegistries(variables?: DocumentRegistriesQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DocumentRegistriesQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<DocumentRegistriesQuery>(DocumentRegistriesDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'documentRegistries', 'query');
    },
    documentRegistriesWithChildren(variables?: DocumentRegistriesWithChildrenQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DocumentRegistriesWithChildrenQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<DocumentRegistriesWithChildrenQuery>(DocumentRegistriesWithChildrenDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'documentRegistriesWithChildren', 'query');
    },
    allocateProgramNumber(variables: AllocateProgramNumberMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<AllocateProgramNumberMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<AllocateProgramNumberMutation>(AllocateProgramNumberDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'allocateProgramNumber', 'mutation');
    },
    encounterFields(variables: EncounterFieldsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<EncounterFieldsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<EncounterFieldsQuery>(EncounterFieldsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'encounterFields', 'query');
    },
    encounterById(variables: EncounterByIdQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<EncounterByIdQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<EncounterByIdQuery>(EncounterByIdDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'encounterById', 'query');
    },
    encounters(variables: EncountersQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<EncountersQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<EncountersQuery>(EncountersDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'encounters', 'query');
    },
    programEnrolments(variables: ProgramEnrolmentsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<ProgramEnrolmentsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<ProgramEnrolmentsQuery>(ProgramEnrolmentsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'programEnrolments', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockDocumentByNameQuery((req, res, ctx) => {
 *   const { name, storeId } = req.variables;
 *   return res(
 *     ctx.data({ document })
 *   )
 * })
 */
export const mockDocumentByNameQuery = (resolver: ResponseResolver<GraphQLRequest<DocumentByNameQueryVariables>, GraphQLContext<DocumentByNameQuery>, any>) =>
  graphql.query<DocumentByNameQuery, DocumentByNameQueryVariables>(
    'documentByName',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockDocumentsQuery((req, res, ctx) => {
 *   const { storeId, page, filter, sort } = req.variables;
 *   return res(
 *     ctx.data({ documents })
 *   )
 * })
 */
export const mockDocumentsQuery = (resolver: ResponseResolver<GraphQLRequest<DocumentsQueryVariables>, GraphQLContext<DocumentsQuery>, any>) =>
  graphql.query<DocumentsQuery, DocumentsQueryVariables>(
    'documents',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockDocumentRegistriesQuery((req, res, ctx) => {
 *   const { filter, sort } = req.variables;
 *   return res(
 *     ctx.data({ documentRegistries })
 *   )
 * })
 */
export const mockDocumentRegistriesQuery = (resolver: ResponseResolver<GraphQLRequest<DocumentRegistriesQueryVariables>, GraphQLContext<DocumentRegistriesQuery>, any>) =>
  graphql.query<DocumentRegistriesQuery, DocumentRegistriesQueryVariables>(
    'documentRegistries',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockDocumentRegistriesWithChildrenQuery((req, res, ctx) => {
 *   const { filter, sort } = req.variables;
 *   return res(
 *     ctx.data({ documentRegistries })
 *   )
 * })
 */
export const mockDocumentRegistriesWithChildrenQuery = (resolver: ResponseResolver<GraphQLRequest<DocumentRegistriesWithChildrenQueryVariables>, GraphQLContext<DocumentRegistriesWithChildrenQuery>, any>) =>
  graphql.query<DocumentRegistriesWithChildrenQuery, DocumentRegistriesWithChildrenQueryVariables>(
    'documentRegistriesWithChildren',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockAllocateProgramNumberMutation((req, res, ctx) => {
 *   const { numberName, storeId } = req.variables;
 *   return res(
 *     ctx.data({ allocateProgramNumber })
 *   )
 * })
 */
export const mockAllocateProgramNumberMutation = (resolver: ResponseResolver<GraphQLRequest<AllocateProgramNumberMutationVariables>, GraphQLContext<AllocateProgramNumberMutation>, any>) =>
  graphql.mutation<AllocateProgramNumberMutation, AllocateProgramNumberMutationVariables>(
    'allocateProgramNumber',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockEncounterFieldsQuery((req, res, ctx) => {
 *   const { storeId, patientId, fields } = req.variables;
 *   return res(
 *     ctx.data({ encounterFields })
 *   )
 * })
 */
export const mockEncounterFieldsQuery = (resolver: ResponseResolver<GraphQLRequest<EncounterFieldsQueryVariables>, GraphQLContext<EncounterFieldsQuery>, any>) =>
  graphql.query<EncounterFieldsQuery, EncounterFieldsQueryVariables>(
    'encounterFields',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockEncounterByIdQuery((req, res, ctx) => {
 *   const { storeId, encounterId } = req.variables;
 *   return res(
 *     ctx.data({ encounters })
 *   )
 * })
 */
export const mockEncounterByIdQuery = (resolver: ResponseResolver<GraphQLRequest<EncounterByIdQueryVariables>, GraphQLContext<EncounterByIdQuery>, any>) =>
  graphql.query<EncounterByIdQuery, EncounterByIdQueryVariables>(
    'encounterById',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockEncountersQuery((req, res, ctx) => {
 *   const { storeId, key, desc, filter, page } = req.variables;
 *   return res(
 *     ctx.data({ encounters })
 *   )
 * })
 */
export const mockEncountersQuery = (resolver: ResponseResolver<GraphQLRequest<EncountersQueryVariables>, GraphQLContext<EncountersQuery>, any>) =>
  graphql.query<EncountersQuery, EncountersQueryVariables>(
    'encounters',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockProgramEnrolmentsQuery((req, res, ctx) => {
 *   const { storeId, key, desc, filter, eventTime } = req.variables;
 *   return res(
 *     ctx.data({ programEnrolments })
 *   )
 * })
 */
export const mockProgramEnrolmentsQuery = (resolver: ResponseResolver<GraphQLRequest<ProgramEnrolmentsQueryVariables>, GraphQLContext<ProgramEnrolmentsQuery>, any>) =>
  graphql.query<ProgramEnrolmentsQuery, ProgramEnrolmentsQueryVariables>(
    'programEnrolments',
    resolver
  )
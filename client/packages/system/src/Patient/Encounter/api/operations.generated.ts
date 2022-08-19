import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type EncounterDocumentRegistryFragment = { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any };

export type EncounterDocumentFragment = { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null };

export type EncounterFragment = { __typename: 'EncounterNode', type: string, name: string, status?: Types.EncounterNodeStatus | null, patientId: string, program: string, startDatetime: string, endDatetime?: string | null, document: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } };

export type EncountersQueryVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  key?: Types.InputMaybe<Types.EncounterSortFieldInput>;
  desc?: Types.InputMaybe<Types.Scalars['Boolean']>;
  filter?: Types.InputMaybe<Types.EncounterFilterInput>;
}>;


export type EncountersQuery = { __typename: 'FullQuery', encounters: { __typename: 'EncounterConnector', totalCount: number, nodes: Array<{ __typename: 'EncounterNode', type: string, name: string, status?: Types.EncounterNodeStatus | null, patientId: string, program: string, startDatetime: string, endDatetime?: string | null, document: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } }> } };

export type EncounterDocumentRegistriesQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.DocumentRegistryFilterInput>;
}>;


export type EncounterDocumentRegistriesQuery = { __typename: 'FullQuery', documentRegistries: { __typename: 'DocumentRegistryConnector', totalCount: number, nodes: Array<{ __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any }> } };

export type InsertEncounterMutationVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  input: Types.InsertEncounterInput;
}>;


export type InsertEncounterMutation = { __typename: 'FullMutation', insertEncounter: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } };

export type UpdateEncounterMutationVariables = Types.Exact<{
  storeId: Types.Scalars['String'];
  input: Types.UpdateEncounterInput;
}>;


export type UpdateEncounterMutation = { __typename: 'FullMutation', updateEncounter: { __typename: 'DocumentNode', id: string, name: string, parents: Array<string>, author: string, timestamp: string, type: string, data: any, documentRegistry?: { __typename: 'DocumentRegistryNode', uiSchemaType: string, documentType: string, context: Types.DocumentRegistryNodeContext, formSchemaId: string, jsonSchema: any, uiSchema: any } | null } };

export const EncounterDocumentRegistryFragmentDoc = gql`
    fragment EncounterDocumentRegistry on DocumentRegistryNode {
  uiSchemaType
  documentType
  context
  formSchemaId
  jsonSchema
  uiSchema
}
    `;
export const EncounterDocumentFragmentDoc = gql`
    fragment EncounterDocument on DocumentNode {
  id
  name
  parents
  author
  timestamp
  type
  data
  documentRegistry {
    ...EncounterDocumentRegistry
  }
}
    ${EncounterDocumentRegistryFragmentDoc}`;
export const EncounterFragmentDoc = gql`
    fragment Encounter on EncounterNode {
  type
  name
  status
  patientId
  program
  startDatetime
  endDatetime
  document {
    ...EncounterDocument
  }
}
    ${EncounterDocumentFragmentDoc}`;
export const EncountersDocument = gql`
    query encounters($storeId: String!, $key: EncounterSortFieldInput, $desc: Boolean, $filter: EncounterFilterInput) {
  encounters(storeId: $storeId, sort: {key: $key, desc: $desc}, filter: $filter) {
    ... on EncounterConnector {
      nodes {
        ...Encounter
      }
      totalCount
    }
  }
}
    ${EncounterFragmentDoc}`;
export const EncounterDocumentRegistriesDocument = gql`
    query encounterDocumentRegistries($filter: DocumentRegistryFilterInput) {
  documentRegistries(filter: $filter) {
    ... on DocumentRegistryConnector {
      nodes {
        ...EncounterDocumentRegistry
      }
      totalCount
    }
  }
}
    ${EncounterDocumentRegistryFragmentDoc}`;
export const InsertEncounterDocument = gql`
    mutation insertEncounter($storeId: String!, $input: InsertEncounterInput!) {
  insertEncounter(storeId: $storeId, input: $input) {
    ... on DocumentNode {
      __typename
      ...EncounterDocument
    }
  }
}
    ${EncounterDocumentFragmentDoc}`;
export const UpdateEncounterDocument = gql`
    mutation updateEncounter($storeId: String!, $input: UpdateEncounterInput!) {
  updateEncounter(storeId: $storeId, input: $input) {
    ... on DocumentNode {
      __typename
      ...EncounterDocument
    }
  }
}
    ${EncounterDocumentFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    encounters(variables: EncountersQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<EncountersQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<EncountersQuery>(EncountersDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'encounters', 'query');
    },
    encounterDocumentRegistries(variables?: EncounterDocumentRegistriesQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<EncounterDocumentRegistriesQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<EncounterDocumentRegistriesQuery>(EncounterDocumentRegistriesDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'encounterDocumentRegistries', 'query');
    },
    insertEncounter(variables: InsertEncounterMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<InsertEncounterMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<InsertEncounterMutation>(InsertEncounterDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'insertEncounter', 'mutation');
    },
    updateEncounter(variables: UpdateEncounterMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateEncounterMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateEncounterMutation>(UpdateEncounterDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateEncounter', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockEncountersQuery((req, res, ctx) => {
 *   const { storeId, key, desc, filter } = req.variables;
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
 * mockEncounterDocumentRegistriesQuery((req, res, ctx) => {
 *   const { filter } = req.variables;
 *   return res(
 *     ctx.data({ documentRegistries })
 *   )
 * })
 */
export const mockEncounterDocumentRegistriesQuery = (resolver: ResponseResolver<GraphQLRequest<EncounterDocumentRegistriesQueryVariables>, GraphQLContext<EncounterDocumentRegistriesQuery>, any>) =>
  graphql.query<EncounterDocumentRegistriesQuery, EncounterDocumentRegistriesQueryVariables>(
    'encounterDocumentRegistries',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockInsertEncounterMutation((req, res, ctx) => {
 *   const { storeId, input } = req.variables;
 *   return res(
 *     ctx.data({ insertEncounter })
 *   )
 * })
 */
export const mockInsertEncounterMutation = (resolver: ResponseResolver<GraphQLRequest<InsertEncounterMutationVariables>, GraphQLContext<InsertEncounterMutation>, any>) =>
  graphql.mutation<InsertEncounterMutation, InsertEncounterMutationVariables>(
    'insertEncounter',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockUpdateEncounterMutation((req, res, ctx) => {
 *   const { storeId, input } = req.variables;
 *   return res(
 *     ctx.data({ updateEncounter })
 *   )
 * })
 */
export const mockUpdateEncounterMutation = (resolver: ResponseResolver<GraphQLRequest<UpdateEncounterMutationVariables>, GraphQLContext<UpdateEncounterMutation>, any>) =>
  graphql.mutation<UpdateEncounterMutation, UpdateEncounterMutationVariables>(
    'updateEncounter',
    resolver
  )
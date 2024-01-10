import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import { GraphQLClientRequestHeaders } from 'graphql-request/build/cjs/types';
import gql from 'graphql-tag';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type TemperatureBreachFragment = { __typename: 'TemperatureBreachNode', id: string, unacknowledged: boolean, durationMilliseconds: number, endDatetime?: string | null, startDatetime: string, type: Types.TemperatureBreachNodeType, maxOrMinTemperature?: number | null, comment?: string | null, sensor?: { __typename: 'SensorNode', id: string, name: string } | null, location?: { __typename: 'LocationNode', name: string } | null };

export type Temperature_BreachesQueryVariables = Types.Exact<{
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.TemperatureBreachSortInput> | Types.TemperatureBreachSortInput>;
  filter?: Types.InputMaybe<Types.TemperatureBreachFilterInput>;
  storeId: Types.Scalars['String']['input'];
}>;


export type Temperature_BreachesQuery = { __typename: 'Queries', temperatureBreaches: { __typename: 'TemperatureBreachConnector', totalCount: number, nodes: Array<{ __typename: 'TemperatureBreachNode', id: string, unacknowledged: boolean, durationMilliseconds: number, endDatetime?: string | null, startDatetime: string, type: Types.TemperatureBreachNodeType, maxOrMinTemperature?: number | null, comment?: string | null, sensor?: { __typename: 'SensorNode', id: string, name: string } | null, location?: { __typename: 'LocationNode', name: string } | null }> } };

export type UpdateTemperatureBreachMutationVariables = Types.Exact<{
  input: Types.UpdateTemperatureBreachInput;
  storeId: Types.Scalars['String']['input'];
}>;


export type UpdateTemperatureBreachMutation = { __typename: 'Mutations', updateTemperatureBreach: { __typename: 'TemperatureBreachNode', id: string, comment?: string | null, unacknowledged: boolean } | { __typename: 'UpdateTemperatureBreachError', error: { __typename: 'CommentNotProvided', description: string } | { __typename: 'RecordNotFound', description: string } } };

export const TemperatureBreachFragmentDoc = gql`
    fragment TemperatureBreach on TemperatureBreachNode {
  __typename
  id
  unacknowledged
  durationMilliseconds
  endDatetime
  startDatetime
  type
  maxOrMinTemperature
  comment
  sensor {
    id
    name
  }
  location {
    name
  }
}
    `;
export const Temperature_BreachesDocument = gql`
    query temperature_breaches($page: PaginationInput, $sort: [TemperatureBreachSortInput!], $filter: TemperatureBreachFilterInput, $storeId: String!) {
  temperatureBreaches(
    page: $page
    sort: $sort
    filter: $filter
    storeId: $storeId
  ) {
    ... on TemperatureBreachConnector {
      totalCount
      nodes {
        ...TemperatureBreach
      }
    }
  }
}
    ${TemperatureBreachFragmentDoc}`;
export const UpdateTemperatureBreachDocument = gql`
    mutation updateTemperatureBreach($input: UpdateTemperatureBreachInput!, $storeId: String!) {
  updateTemperatureBreach(input: $input, storeId: $storeId) {
    ... on TemperatureBreachNode {
      id
      comment
      unacknowledged
    }
    ... on UpdateTemperatureBreachError {
      __typename
      error {
        description
        ... on RecordNotFound {
          __typename
          description
        }
        ... on CommentNotProvided {
          __typename
          description
        }
      }
    }
  }
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    temperature_breaches(variables: Temperature_BreachesQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<Temperature_BreachesQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<Temperature_BreachesQuery>(Temperature_BreachesDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'temperature_breaches', 'query');
    },
    updateTemperatureBreach(variables: UpdateTemperatureBreachMutationVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<UpdateTemperatureBreachMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateTemperatureBreachMutation>(UpdateTemperatureBreachDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateTemperatureBreach', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockTemperatureBreachesQuery((req, res, ctx) => {
 *   const { page, sort, filter, storeId } = req.variables;
 *   return res(
 *     ctx.data({ temperatureBreaches })
 *   )
 * })
 */
export const mockTemperatureBreachesQuery = (resolver: ResponseResolver<GraphQLRequest<Temperature_BreachesQueryVariables>, GraphQLContext<Temperature_BreachesQuery>, any>) =>
  graphql.query<Temperature_BreachesQuery, Temperature_BreachesQueryVariables>(
    'temperature_breaches',
    resolver
  )

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockUpdateTemperatureBreachMutation((req, res, ctx) => {
 *   const { input, storeId } = req.variables;
 *   return res(
 *     ctx.data({ updateTemperatureBreach })
 *   )
 * })
 */
export const mockUpdateTemperatureBreachMutation = (resolver: ResponseResolver<GraphQLRequest<UpdateTemperatureBreachMutationVariables>, GraphQLContext<UpdateTemperatureBreachMutation>, any>) =>
  graphql.mutation<UpdateTemperatureBreachMutation, UpdateTemperatureBreachMutationVariables>(
    'updateTemperatureBreach',
    resolver
  )

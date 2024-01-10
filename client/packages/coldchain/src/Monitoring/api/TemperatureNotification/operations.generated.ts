import * as Types from '@openmsupply-client/common';

import { GraphQLClient } from 'graphql-request';
import { GraphQLClientRequestHeaders } from 'graphql-request/build/cjs/types';
import gql from 'graphql-tag';
import { graphql, ResponseResolver, GraphQLRequest, GraphQLContext } from 'msw'
export type TemperatureNotificationBreachFragment = { __typename: 'TemperatureBreachNode', id: string, startDatetime: string, maxOrMinTemperature?: number | null, sensor?: { __typename: 'SensorNode', id: string, name: string } | null, location?: { __typename: 'LocationNode', name: string } | null };

export type TemperatureExcursionFragment = { __typename: 'TemperatureBreachNode', id: string, startDatetime: string, maxOrMinTemperature?: number | null, sensor?: { __typename: 'SensorNode', id: string, name: string } | null, location?: { __typename: 'LocationNode', name: string } | null };

export type TemperatureNotificationsQueryVariables = Types.Exact<{
  page?: Types.InputMaybe<Types.PaginationInput>;
  storeId: Types.Scalars['String']['input'];
}>;


export type TemperatureNotificationsQuery = { __typename: 'Queries', temperatureNotifications: { __typename: 'TemperatureNotificationConnector', breaches: Array<{ __typename: 'TemperatureBreachNode', id: string, startDatetime: string, maxOrMinTemperature?: number | null, sensor?: { __typename: 'SensorNode', id: string, name: string } | null, location?: { __typename: 'LocationNode', name: string } | null }>, excursions: Array<{ __typename: 'TemperatureExcursionNode' }> } };

export const TemperatureNotificationBreachFragmentDoc = gql`
    fragment TemperatureNotificationBreach on TemperatureBreachNode {
  __typename
  id
  startDatetime
  maxOrMinTemperature
  sensor {
    id
    name
  }
  location {
    name
  }
}
    `;
export const TemperatureExcursionFragmentDoc = gql`
    fragment TemperatureExcursion on TemperatureBreachNode {
  __typename
  id
  startDatetime
  maxOrMinTemperature
  sensor {
    id
    name
  }
  location {
    name
  }
}
    `;
export const TemperatureNotificationsDocument = gql`
    query temperatureNotifications($page: PaginationInput, $storeId: String!) {
  temperatureNotifications(page: $page, storeId: $storeId) {
    ... on TemperatureNotificationConnector {
      breaches {
        ...TemperatureNotificationBreach
      }
      excursions {
        ...TemperatureExcursion
      }
    }
  }
}
    ${TemperatureNotificationBreachFragmentDoc}
${TemperatureExcursionFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    temperatureNotifications(variables: TemperatureNotificationsQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<TemperatureNotificationsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<TemperatureNotificationsQuery>(TemperatureNotificationsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'temperatureNotifications', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

/**
 * @param resolver a function that accepts a captured request and may return a mocked response.
 * @see https://mswjs.io/docs/basics/response-resolver
 * @example
 * mockTemperatureNotificationsQuery((req, res, ctx) => {
 *   const { page, storeId } = req.variables;
 *   return res(
 *     ctx.data({ temperatureNotifications })
 *   )
 * })
 */
export const mockTemperatureNotificationsQuery = (resolver: ResponseResolver<GraphQLRequest<TemperatureNotificationsQueryVariables>, GraphQLContext<TemperatureNotificationsQuery>, any>) =>
  graphql.query<TemperatureNotificationsQuery, TemperatureNotificationsQueryVariables>(
    'temperatureNotifications',
    resolver
  )

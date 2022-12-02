import { GraphQLClient } from "graphql-request";
import * as Dom from "graphql-request/dist/types.dom";
import gql from "graphql-tag";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>;
};
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type NewsFeedStatus = {
  __typename?: "NewsFeedStatus";
  completedAt?: Maybe<Scalars["Int"]>;
};

export type NewsFeedUrl = {
  __typename?: "NewsFeedUrl";
  createdAt: Scalars["Int"];
  description: Scalars["String"];
  displayUrl: Scalars["String"];
  expandedUrlHost: Scalars["String"];
  expandedUrlParsed: Scalars["String"];
  firstReferencedByUsername: Scalars["String"];
  numReferences: Scalars["Int"];
  previewImageThumbnailUrl?: Maybe<Scalars["String"]>;
  previewImageUrl?: Maybe<Scalars["String"]>;
  title: Scalars["String"];
  urlId: Scalars["Int"];
  urlScore: Scalars["Int"];
};

export type NewsFeedUrlReference = {
  __typename?: "NewsFeedUrlReference";
  authorUsername: Scalars["String"];
  retweetedByUsernames: Array<Scalars["String"]>;
  tweetCreatedAtStr: Scalars["String"];
  tweetId: Scalars["String"];
  tweetText: Scalars["String"];
  urlId: Scalars["Int"];
};

export type Query = {
  __typename?: "Query";
  newsFeedStatus: NewsFeedStatus;
  newsFeedUrl: NewsFeedUrl;
  newsFeedUrlReferences: Array<NewsFeedUrlReference>;
  newsFeedUrls: Array<NewsFeedUrl>;
};

export type QueryNewsFeedUrlArgs = {
  urlId: Scalars["Int"];
};

export type QueryNewsFeedUrlReferencesArgs = {
  urlId: Scalars["Int"];
};

export type GetNewsFeedUrlAndReferencesQueryVariables = Exact<{
  urlId: Scalars["Int"];
}>;

export type GetNewsFeedUrlAndReferencesQuery = {
  __typename?: "Query";
  newsFeedUrl: {
    __typename?: "NewsFeedUrl";
    urlId: number;
    urlScore: number;
    numReferences: number;
    firstReferencedByUsername: string;
    createdAt: number;
    title: string;
    description: string;
    expandedUrlParsed: string;
    expandedUrlHost: string;
    previewImageThumbnailUrl?: string | null;
    previewImageUrl?: string | null;
    displayUrl: string;
  };
  newsFeedUrlReferences: Array<{
    __typename?: "NewsFeedUrlReference";
    tweetId: string;
    tweetText: string;
    tweetCreatedAtStr: string;
    authorUsername: string;
    retweetedByUsernames: Array<string>;
  }>;
};

export type GetNewsFeedUrlsAndNewsFeedStatusQueryVariables = Exact<{
  [key: string]: never;
}>;

export type GetNewsFeedUrlsAndNewsFeedStatusQuery = {
  __typename?: "Query";
  newsFeedUrls: Array<{
    __typename?: "NewsFeedUrl";
    urlId: number;
    urlScore: number;
    numReferences: number;
    firstReferencedByUsername: string;
    createdAt: number;
    title: string;
    description: string;
    expandedUrlParsed: string;
    expandedUrlHost: string;
    previewImageThumbnailUrl?: string | null;
    previewImageUrl?: string | null;
    displayUrl: string;
  }>;
  newsFeedStatus: {
    __typename?: "NewsFeedStatus";
    completedAt?: number | null;
  };
};

export const GetNewsFeedUrlAndReferencesDocument = gql`
  query GetNewsFeedUrlAndReferences($urlId: Int!) {
    newsFeedUrl(urlId: $urlId) {
      urlId
      urlScore
      numReferences
      firstReferencedByUsername
      createdAt
      title
      description
      expandedUrlParsed
      expandedUrlHost
      previewImageThumbnailUrl
      previewImageUrl
      displayUrl
    }
    newsFeedUrlReferences(urlId: $urlId) {
      tweetId
      tweetText
      tweetCreatedAtStr
      authorUsername
      retweetedByUsernames
    }
  }
`;
export const GetNewsFeedUrlsAndNewsFeedStatusDocument = gql`
  query GetNewsFeedUrlsAndNewsFeedStatus {
    newsFeedUrls {
      urlId
      urlScore
      numReferences
      firstReferencedByUsername
      createdAt
      title
      description
      expandedUrlParsed
      expandedUrlHost
      previewImageThumbnailUrl
      previewImageUrl
      displayUrl
    }
    newsFeedStatus {
      completedAt
    }
  }
`;

export type SdkFunctionWrapper = <T>(
  action: (requestHeaders?: Record<string, string>) => Promise<T>,
  operationName: string,
  operationType?: string
) => Promise<T>;

const defaultWrapper: SdkFunctionWrapper = (
  action,
  _operationName,
  _operationType
) => action();

export function getSdk(
  client: GraphQLClient,
  withWrapper: SdkFunctionWrapper = defaultWrapper
) {
  return {
    GetNewsFeedUrlAndReferences(
      variables: GetNewsFeedUrlAndReferencesQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetNewsFeedUrlAndReferencesQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetNewsFeedUrlAndReferencesQuery>(
            GetNewsFeedUrlAndReferencesDocument,
            variables,
            { ...requestHeaders, ...wrappedRequestHeaders }
          ),
        "GetNewsFeedUrlAndReferences",
        "query"
      );
    },
    GetNewsFeedUrlsAndNewsFeedStatus(
      variables?: GetNewsFeedUrlsAndNewsFeedStatusQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetNewsFeedUrlsAndNewsFeedStatusQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetNewsFeedUrlsAndNewsFeedStatusQuery>(
            GetNewsFeedUrlsAndNewsFeedStatusDocument,
            variables,
            { ...requestHeaders, ...wrappedRequestHeaders }
          ),
        "GetNewsFeedUrlsAndNewsFeedStatus",
        "query"
      );
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;

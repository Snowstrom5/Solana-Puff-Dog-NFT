import { GraphQLClient } from "graphql-request";

const graphqlApiUrl =
  process.env.GRAPHQL_API_URL || "http://0.0.0.0:8000/graphql";
export const graphQLClient = new GraphQLClient(graphqlApiUrl);

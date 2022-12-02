import type { NextPage } from "next";
import NewsFeedUrlContent from "components/feature/news_feed_url_content";
import {
  NewsFeedUrlReference,
  NewsFeedUrl,
  getSdk
} from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

interface NewsFeedUrlPageProps {
  newsFeedUrl: NewsFeedUrl;
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

const NewsFeedUrlPage: NextPage<NewsFeedUrlPageProps> = ({
  newsFeedUrl,
  newsFeedUrlReferences
}) => {
  return (
    <>
      <Meta />
      <NavBar pageRoute="/" />
      <NewsFeedUrlContent
        newsFeedUrl={newsFeedUrl}
        newsFeedUrlReferences={newsFeedUrlReferences}
      />
    </>
  );
};

export async function getServerSideProps(context: any) {
  const { url_id } = context.query;
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrlAndReferences({
    urlId: Number(url_id)
  });
  return {
    props: {
      newsFeedUrl: response.newsFeedUrl,
      newsFeedUrlReferences: response.newsFeedUrlReferences
    }
  };
}

export default NewsFeedUrlPage;

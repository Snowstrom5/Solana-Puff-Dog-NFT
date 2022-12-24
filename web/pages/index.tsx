import type { NextPage } from "next";
import NewsContent from "components/feature/news_content";
import { NewsFeedUrl, NewsFeedStatus, getSdk } from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

interface NewsPageProps {
  newsFeedUrls: NewsFeedUrl[];
  newsFeedStatus?: NewsFeedStatus;
}

const NewsPage: NextPage<NewsPageProps> = ({
  newsFeedUrls,
  newsFeedStatus
}) => {
  return (
    <>
      <Meta />
      <NavBar pageRoute="/" />
      <NewsContent
        newsFeedUrls={newsFeedUrls}
        newsFeedStatus={newsFeedStatus}
      />
    </>
  );
};

export async function getServerSideProps(context: any) {
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrlsAndNewsFeedStatus();
  return {
    props: {
      newsFeedUrls: response.newsFeedUrls,
      newsFeedStatus: response.newsFeedStatus
    }
  };
}

export default NewsPage;

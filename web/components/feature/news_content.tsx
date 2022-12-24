import { NewsFeedUrl, NewsFeedStatus } from "graphql/generated/graphql";
import Link from "next/link";
import NewsHeader from "components/generic/news_header";
import { sharedByText } from "app/util";
import { timeSince } from "app/time";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
  newsFeedStatus?: NewsFeedStatus;
}

export default function NewsContent(props: NewsContentProps) {
  let lastUpdated = "";
  if (props.newsFeedStatus?.completedAt) {
    lastUpdated = timeSince(props.newsFeedStatus.completedAt);
  }

  return (
    <>
      <NewsHeader
        title="News"
        subtitle="Trending climate related articles shared by leading climate scientists, organizations, journalists and activists."
        lastUpdated={lastUpdated}
      />
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        <ul>
          {props.newsFeedUrls &&
            props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl) => {
              return (
                <li
                  className="my-4 grid grid-cols-12"
                  key={newsFeedUrl.expandedUrlParsed}
                >
                  {/* Title */}
                  <div className="col-span-10">
                    <a
                      href={newsFeedUrl.expandedUrlParsed}
                      className="hover:underline"
                    >
                      <div className="flex flex-row">
                        <p className="text-base mr-1">
                          <b>{newsFeedUrl?.title}</b> (
                          {newsFeedUrl?.expandedUrlHost})
                        </p>
                      </div>
                    </a>

                    {/* Shares */}
                    <p className="text-base text-gray-500 mt-1">
                      <Link
                        href={{
                          pathname: "/news_feed/[url_slug]",
                          query: { url_slug: newsFeedUrl.urlSlug }
                        }}
                        className="hover:underline"
                      >
                        {sharedByText(newsFeedUrl)}
                      </Link>
                    </p>
                  </div>

                  {/* Image preview */}
                  <div className="col-span-2">
                    <a
                      href={newsFeedUrl.expandedUrlParsed}
                      className="hover:underline"
                    >
                      <img
                        className="mx-auto h-15 w-15 rounded lg:h-20 lg:w-20 lg:rounded-md"
                        src={
                          newsFeedUrl.previewImageThumbnailUrl
                            ? newsFeedUrl.previewImageThumbnailUrl
                            : "/news_article_placeholder.png"
                        }
                        alt=""
                      />
                    </a>
                  </div>
                </li>
              );
            })}
        </ul>
      </div>
    </>
  );
}

import BackButtonHeader from "components/generic/back_button_header";
import { NewsFeedUrlReference, NewsFeedUrl } from "graphql/generated/graphql";
import Link from "next/link";
import NewsFeedUrlReferences from "./news_feed_url_references";

interface NewsFeedUrlContentProps {
  newsFeedUrl: NewsFeedUrl;
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsFeedUrlContent(props: NewsFeedUrlContentProps) {
  return (
    <>
      <BackButtonHeader />
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        <div
          className="my-4 grid grid-cols-12"
          key={props.newsFeedUrl.expandedUrlParsed}
        >
          {/* Title */}
          <div className="col-span-10">
            <div className="flex flex-row">
              <a
                className="hover:underline"
                href={props.newsFeedUrl?.expandedUrlParsed}
              >
                <p className="text-xl font-bold">{props.newsFeedUrl?.title}</p>
              </a>
            </div>
            <div className="flex flex-row mt-2">
              <p className="text-base">{props.newsFeedUrl?.description}</p>
            </div>

            <div className="flex flex-row mt-2">
              {/* link */}
              <a
                className="text-blue-600 text-sm hover:underline"
                href={props.newsFeedUrl?.expandedUrlParsed}
              >
                {props.newsFeedUrl?.displayUrl} &rarr;
              </a>
            </div>
          </div>

          {/* Image preview */}
          <div className="col-span-2">
            <Link
              href={{
                pathname: "/news_feed/[url_id]",
                query: { url_id: props.newsFeedUrl.urlId }
              }}
              className=" hover:underline"
            >
              <img
                className="mx-auto h-30 w-30 rounded lg:h-30 lg:w-30 lg:rounded-md"
                src={
                  props.newsFeedUrl.previewImageThumbnailUrl
                    ? props.newsFeedUrl.previewImageThumbnailUrl
                    : "news_article_placeholder.png"
                }
                alt="TODO"
              />
            </Link>
          </div>
        </div>

        <h3 className="text-lg font-bold text-gray-900 text-left mt-6 mb-2">
          Tweets:
        </h3>

        {props.newsFeedUrlReferences.length > 0 && (
          <NewsFeedUrlReferences
            newsFeedUrlReferences={props.newsFeedUrlReferences}
          />
        )}
      </div>
    </>
  );
}

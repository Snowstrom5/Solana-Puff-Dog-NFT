import { retweetedByText, unescapeHTML } from "app/util";
import { NewsFeedUrlReference } from "graphql/generated/graphql";

interface NewsFeedUrlReferencesProps {
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsFeedUrlDirectReferences(
  props: NewsFeedUrlReferencesProps
) {
  return (
    <>
      <div className="grid lg:grid-cols-2 sm:grid-cols-1 gap-4">
        {props.newsFeedUrlReferences.map(
          (newsFeedUrlReference: NewsFeedUrlReference) => {
            return (
              <div
                key={newsFeedUrlReference.tweetId}
                className="border-solid border-2 border-gray-300 rounded-md p-4"
              >
                <div className="flex flex-row">
                  <a
                    href={`https://twitter.com/${newsFeedUrlReference.authorUsername}`}
                  >
                    <p className="text-m font-medium">
                      @{newsFeedUrlReference.authorUsername}
                    </p>
                  </a>{" "}
                  <a
                    href={`https://twitter.com/${newsFeedUrlReference.authorUsername}/status/${newsFeedUrlReference.tweetId}`}
                    className="ml-2 hover:underline"
                  >
                    <img
                      className="mx-auto h-5 w-5"
                      src={"/twitter_icon.svg"}
                      alt="twitter_icon"
                    />
                  </a>
                </div>

                <p className="text-m mt-2">
                  {unescapeHTML(newsFeedUrlReference.tweetText)}
                </p>

                {newsFeedUrlReference.retweetedByUsernames.length > 0 && (
                  <div className="flex flex-row mt-2">
                    <img
                      className="h-4 w-4 mt-1 mr-1"
                      src={"/retweet_icon.png"}
                      alt="retweet_icon"
                    />
                    <p className="text-m font-light italic text-gray-800 ">
                      {retweetedByText(
                        newsFeedUrlReference.retweetedByUsernames
                      )}
                    </p>
                  </div>
                )}
              </div>
            );
          }
        )}
      </div>
    </>
  );
}

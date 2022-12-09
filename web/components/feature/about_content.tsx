import NewsHeader from "components/generic/news_header";
import Link from "next/link";

export default function AboutContent() {
  return (
    <>
      <NewsHeader title="About" subtitle="Who we are" />
      <div className="container w-full md:max-w-3xl mx-auto">
        {/* Content */}
        <div className="relative py-2">
          <div className="mx-auto max-w-md px-4 sm:max-w-3xl">
            <p className="mx-auto mt-5 text-lg text-gray-700">
              <Link
                href="https://climatenews.app"
                className="font-medium hover:underline"
              >
                Climate News
              </Link>{" "}
              is an open source project aiming to improve climate related
              education.
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              The news feed shows climate related articles shared by leading
              climate scientists, organizations, journalists and activists on
              Twitter.
            </p>

          <p className="mt-8 text-xl font-extrabold tracking-tight text-gray-900">
            Twitter Lists
          </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              3500+ Twitter accounts are imported from the lists below to
              generate the user list:
            </p>
            <ul className="mx-auto ml-5 mt-5 max-w-prose list-disc">
              <li className="mx-auto mt-2 text-lg text-gray-700">
                <Link
                  href="https://twitter.com/i/lists/1586920047964205057"
                  className="font-medium hover:underline"
                >
                  Climate News
                </Link>
                {" by "}
                <Link
                  href="https://twitter.com/climatenews_app"
                  className="font-medium hover:underline"
                >
                  @climatenews_app
                </Link>
              </li>
              <li className="mx-auto mt-1  text-lg text-gray-700">
                <Link
                  href="https://twitter.com/i/lists/1053067173961326594"
                  className="font-medium hover:underline"
                >
                  scientists who do climate
                </Link>
                {" by "}
                <Link
                  href="https://twitter.com/KHayhoe"
                  className="font-medium hover:underline"
                >
                  @KHayhoe
                </Link>
              </li>
              <li className="mx-auto mt-1  text-lg text-gray-700">
                <Link
                  href="https://twitter.com/i/lists/1308140854524162059"
                  className="font-medium hover:underline"
                >
                  Climate change
                </Link>
                {" by "}
                <Link
                  href="https://twitter.com/TwitterMoments"
                  className="font-medium hover:underline"
                >
                  @TwitterMoments
                </Link>
                .
              </li>
            </ul>
            <p className="mt-8 text-xl font-extrabold tracking-tight text-gray-900">
            News Feed
          </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              The news feed uses a ranking algorithm to find trending articles.
            </p>

            <p className="mx-auto mt-5  text-lg text-gray-700">
              Users are scored based on their follower count, the number of
              lists they appear in, and the amount of times they have been
              referenced by other users.
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              The article&apos;s score is based on the score of the users that
              shared it. It will gradually decrease over time.
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              The{" "}
              <Link
                href="https://developer.twitter.com/en/docs/twitter-api"
                className="font-medium hover:underline"
              >
                Twitter API
              </Link>{" "}
              is used to keep track of articles shared by users. The{" "}
              <Link
                href="https://openai.com"
                className="font-medium hover:underline"
              >
                OpenAI GPT3 API
              </Link>{" "}
              is used to classify each article, to ensure only climate related
              articles appear in the news feed.
            </p>
            <p className="mt-8 text-xl font-extrabold tracking-tight text-gray-900">
            Source Code
          </p>
            <p className="mx-auto mt-5 text-lg text-gray-700">
              Our source code is available on{" "}
              <Link
                href="https://github.com/climatenews/services"
                className="font-medium hover:underline"
              >
                GitHub
              </Link>
              {". "}
              Contributions are welcome!
            </p>
            <p className="mt-8 text-xl font-extrabold tracking-tight text-gray-900">
            About Us
          </p>
            <p className="mx-auto mt-5  text-lg text-gray-700">
              Made with 💚 in Nanaimo, Canada by{" "}
              <Link
                href="https://twitter.com/patrickf_ca"
                className="font-medium hover:underline"
              >
                Patrick Fitzgerald
              </Link>
              {"."}
            </p>
          </div>
        </div>
      </div>
    </>
  );
}

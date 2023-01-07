import type { NextPage } from "next";
import Head from "next/head";

interface Props {
  title?: string;
  keywords?: string;
  description?: string;
  websiteUrl?: string;
  type?: string;
  siteName?: string;
  imageSource?: string;
  imageType?: string;
  imageAltText?: string;
  imageWidth?: string;
  imageHeight?: string;
  twitterCard?: string;
  twitterName?: string;
}

const Meta: NextPage<Props> = ({
  title,
  keywords,
  description,
  websiteUrl,
  type,
  siteName,
  imageSource,
  imageType,
  imageAltText,
  imageWidth,
  imageHeight,
  twitterCard,
  twitterName
}) => {
  return (
    <Head>
      {/* General */}
      <meta
        name="viewport"
        content="width=device-width, initial-scale=1"
      ></meta>
      <meta charSet="utf-8"></meta>
      <meta name="robots" content="follow, index" />
      <meta name="keywords" content={keywords}></meta>
      <link rel="canonical" href={websiteUrl} />
      <meta name="description" content={description} />

      {/* Favicon */}
      <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
      <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
      <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
      <link rel="manifest" href="/site.webmanifest" />
      <meta name="apple-mobile-web-app-title" content="Climate News" />
      <meta name="application-name" content="Climate News" />
      <meta name="msapplication-TileColor" content="#ffffff" />
      <meta name="theme-color" content="#ffffff" />

      <title>{title}</title>
      {/* Open Graph */}
      <meta property="og:url" content={websiteUrl} />
      <meta property="og:type" content={type} />
      <meta property="og:site_name" content={siteName} />
      <meta property="og:description" content={description} />
      <meta property="og:title" content={title} />
      <meta property="og:image" content={imageSource} />
      <meta property="og:image:type" content={imageType} />
      <meta property="og:image:alt" content={imageAltText} />
      <meta property="og:image:width" content={imageWidth} />
      <meta property="og:image:height" content={imageHeight} />
      {/* Twitter */}
      <meta name="twitter:card" content={twitterCard} />
      <meta name="twitter:site" content={twitterName} />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
      <meta name="twitter:image" content={imageSource} />
      <meta name="twitter:image:alt" content={imageAltText} />
    </Head>
  );
};

export default Meta;

Meta.defaultProps = {
  title: "Climate News",
  keywords: "climate change, climate news, climate science",
  description:
    "Trending climate related articles shared by leading climate scientists, organizations, journalists and activists.",
  imageSource: "https://climatenews.app/og_image.png",
  imageType: "image/png",
  imageAltText: "Climate News",
  imageWidth: "1200",
  imageHeight: "627",
  type: "website",
  websiteUrl: "https://climatenews.app",
  siteName: "Climate News",
  twitterCard: "summary_large_image",
  twitterName: "@climatenews_app"
};

import { BASE_DOMAIN_NAME, Nullable } from "app/util";
import { graphQLClient } from "graphql/client";
import { getSdk } from "graphql/generated/graphql";

export default function SitemapIndex() {}

// Main static sitemap
function generateMainSiteMap(): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
      <url>
        <loc>${BASE_DOMAIN_NAME}</loc>
      </url>
      <url>
        <loc>${BASE_DOMAIN_NAME}/about</loc>
      </url>
    </urlset>`;
}

// Generate a dynamic sitemap from a list of news_feed url_slugs
function generateSiteMap(url_slugs: String[]): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    ${url_slugs
      .map((url_slug) => {
        return `
        <url>
            <loc>${BASE_DOMAIN_NAME}/news_feed/${url_slug}</loc>
        </url>
      `;
      })
      .join("")}
    </urlset>`;
}

export async function getServerSideProps(context: any) {
  const { key } = context.query;
  let sitemap: Nullable<string> = null;
  if (key === "main") {
    sitemap = generateMainSiteMap();
  } else {
    // Example key: 12-2022.xml
    const month_year = key.replace(".xml", "").split("-");
    const sdk = getSdk(graphQLClient);
    const response = await sdk.GetSitemapNewsFeedUrlSlugs({
      month: Number(month_year[0]),
      year: Number(month_year[1])
    });
    sitemap = generateSiteMap(response.sitemapNewsFeedUrlSlugs);
  }

  const res = context.res;
  res.setHeader("Content-Type", "text/xml");
  res.write(sitemap);
  res.end();

  return {
    props: {}
  };
}

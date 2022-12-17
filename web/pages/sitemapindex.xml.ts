import { getMonthsSinceLaunch } from "app/time";
import { BASE_DOMAIN_NAME } from "app/util";
import { GetServerSideProps } from "next";

export default function SitemapIndex() {}

function generateSiteMapIndex() {
  const months = getMonthsSinceLaunch();
  // TODO add lastmod value
  // <lastmod>2004-10-01T18:23:17+00:00</lastmod>
  return `<?xml version="1.0" encoding="UTF-8"?>
    <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
        <sitemap>
            <loc>${BASE_DOMAIN_NAME}/sitemap/main.xml</loc>
        </sitemap>
        ${months
          .map((month) => {
            return `
            <url>
                <loc>${BASE_DOMAIN_NAME}/sitemap/${month.month}-${month.year}.xml</loc>
            </url>
          `;
          })
          .join("")}        
    </sitemapindex>`;
}

export const getServerSideProps: GetServerSideProps = async ({ res }) => {
  const sitemap = generateSiteMapIndex();

  res.setHeader("Content-Type", "text/xml");
  res.write(sitemap);
  res.end();

  return {
    props: {}
  };
};

import "../styles/globals.css";
import type { AppProps } from "next/app";
import Layout from "../components/generic/layout";
import { usePostHog } from "next-use-posthog";

function MyApp({ Component, pageProps }: AppProps) {
  // Posthog Analytics
  usePostHog("phc_ydNVAMi2gNQjqCrcCFTzlH9qzlfCHOD50UaanLHdXzf", {
    api_host: "https://app.posthog.com",
    loaded: (posthog) => {
      // disable in development mode
      if (process.env.NODE_ENV === "development") posthog.opt_out_capturing();
    }
  });

  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  );
}

export default MyApp;

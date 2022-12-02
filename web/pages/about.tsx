import type { NextPage } from "next";
import AboutContent from "components/feature/about_content";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

const AboutPage: NextPage = () => {
  return (
    <>
      <Meta />
      <NavBar pageRoute="/about" />
      <AboutContent />
    </>
  );
};

export default AboutPage;

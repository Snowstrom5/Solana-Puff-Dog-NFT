import Footer from "components/generic/footer";

type LayoutProps = React.PropsWithChildren<{}>;

export default function Layout({ children }: LayoutProps) {
  return (
    <>
      <main className="font-sans leading-normal tracking-normal">
        <div>{children}</div>
      </main>
      <Footer />
    </>
  );
}

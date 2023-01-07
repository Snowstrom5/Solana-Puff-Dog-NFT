interface NewsHeaderProps {
  title?: string;
  subtitle?: string;
  lastUpdated?: string;
}

export default function NewsHeader(props: NewsHeaderProps) {
  return (
    <>
      <div className="py-4 bg-gray-200">
        <div className="container px-4 w-full md:max-w-3xl mx-auto ">
          {props.title ? (
            <h1 className="font-bold font-sans break-normal text-gray-900 text-2xl sm:text-xl">
              {props.title}
            </h1>
          ) : (
            ""
          )}

          {props.subtitle ? (
            <p className="text-sm font-normal text-gray-900 mt-1">
              {props.subtitle}
            </p>
          ) : (
            ""
          )}
          {props.lastUpdated ? (
            <p className="text-sm font-light italic text-gray-700 mt-1 ">
              Last Updated: {props.lastUpdated}
            </p>
          ) : (
            ""
          )}
        </div>
      </div>
    </>
  );
}

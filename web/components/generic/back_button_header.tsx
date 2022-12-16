import { useRouter } from "next/router";
import Link from "next/link";

export default function BackButtonHeader() {
  const router = useRouter();
  return (
    <>
      <div className="py-2 lg:py-6 bg-gray-200">
        <div className="container px-4 w-full md:max-w-3xl mx-auto">
          <Link
            href={{
              pathname: "/"
            }}
            className="hover:underline"
          >
            <p className="text-base font-bold text-gray-700 mt-1">
              &larr; Back
            </p>
          </Link>
        </div>
      </div>
    </>
  );
}

import { useRouter } from "next/router";

export default function BackButtonHeader() {
  const router = useRouter();
  return (
    <>
      <div className="py-2 lg:py-6 bg-gray-200">
        <div className="container px-4 w-full md:max-w-3xl mx-auto">
          <a className="hover:underline" href="#" onClick={() => router.back()}>
            <p className="text-base font-bold text-gray-600 mt-1">
              &larr; Back
            </p>
          </a>
        </div>
      </div>
    </>
  );
}

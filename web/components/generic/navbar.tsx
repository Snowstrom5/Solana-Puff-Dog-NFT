import Link from "next/link";
import { useState } from "react";

const navigation = [
  { name: "News", href: "/" },
  { name: "About", href: "/about" }
];

function classNames(...classes: any[]) {
  return classes.filter(Boolean).join(" ");
}

function isCurrentRoute(pageRoute: string, href: string): boolean {
  return pageRoute === href;
}

interface NavBarProps {
  pageRoute: string;
}

export default function NavBar(props: NavBarProps) {
  const [isNavOpen, setIsNavOpen] = useState(false);
  return (
    <>
      <nav id="header" className="container px-4 w-full md:max-w-3xl mx-auto ">
        <div className="w-full mx-auto flex flex-wrap items-end justify-between py-4">
          <Link href="/">
            <div className="flex flex-1 items-center">
              <div className="flex flex-shrink-0">
                <img
                  className="block h-8 w-auto mb-2"
                  src="/cac_logo.svg"
                  alt="Climate News"
                />
              </div>
              <div className="flex flex-shrink-0">
                <p className="text-gray-900 no-underline hover:no-underline font-extrabold text-lg pl-4 mb-1">
                  Climate News
                </p>
              </div>
            </div>
          </Link>

          {/* Mobile button */}
          <div className="block lg:hidden">
            <button
              id="nav-toggle"
              className="flex items-center px-2 py-2 rounded text-gray-500  hover:text-gray-900 hover:border-green-500 appearance-none focus:outline-none"
              onClick={() => setIsNavOpen((prev) => !prev)}
            >
              <svg
                width="36px"
                height="36px"
                viewBox="0 0 48 48"
                xmlns="http://www.w3.org/2000/svg"
              >
                <title>Hamburger menu</title>
                <path
                  d="M41,14H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,14Z"
                  fill="#000000"
                />
                <path
                  d="M41,26H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,26Z"
                  fill="#000000"
                />
                <path
                  d="M41,38H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,38Z"
                  fill="#000000"
                />
              </svg>
            </button>
          </div>

          <div
            className="w-full flex-grow lg:flex lg:items-center lg:w-auto hidden lg:block mt-2 lg:mt-0 bg-gray-100 md:bg-transparent z-20"
            id="nav-content"
          >
            <ul className="list-reset lg:flex justify-end flex-1 items-center">
              {navigation.map((item) => (
                <li className="mr-3" key={item.name}>
                  <a
                    href={item.href}
                    className={classNames(
                      isCurrentRoute(props.pageRoute, item.href)
                        ? "text-gray-900 font-bold"
                        : "text-gray-600 hover:text-gray-900 hover:text-underline",
                      "py-2 px-4 inline-block no-underline"
                    )}
                    aria-current={
                      isCurrentRoute(props.pageRoute, item.href)
                        ? "page"
                        : undefined
                    }
                  >
                    {item.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        </div>
        <div className={isNavOpen ? "showMenuNav" : "hideMenuNav"}>
          <div
            className="absolute top-0 right-0 px-8 py-8"
            onClick={() => setIsNavOpen(false)}
          >
            <svg
              className="h-8 w-8 text-gray-900"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </div>
          <ul className="flex flex-col items-center justify-around min-h-[150px]">
            {navigation.map((item) => (
              <li
                key={item.href}
                className={classNames(
                  isCurrentRoute(props.pageRoute, item.href)
                    ? "border-b border-gray-400"
                    : "",
                  "text-3xl font-bold my-8"
                )}
              >
                <Link href={item.href}>{item.name}</Link>
              </li>
            ))}
          </ul>
        </div>
      </nav>
      <style>{`
        .hideMenuNav {
          display: none;
        }
        .showMenuNav {
          display: block;
          position: absolute;
          width: 100%;
          height: 100vh;
          top: 0;
          left: 0;
          background: white;
          z-index: 10;
          display: flex;
          flex-direction: column;
          justify-content: space-evenly;
          align-items: center;
        }
      `}</style>
    </>
  );
}

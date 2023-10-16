import Link from "next/link";
import { FC } from "react";
import { Logo } from "./Logo";
import { getBasepathUrl } from "@/lib/utils/baseUrl";

export const NavBar: FC = () => {
  const basePath = getBasepathUrl();
  return (
    <>
      <nav className="bg-white border-b border-gray-200 sticky top-0 w-full">
        <div className="my-3 max-w-screen flex flex-wrap items-center justify-between">
          <div className="max-w-screen-xl flex flex-wrap items-center justify-between p-2">
            <Link href={"/home"}>
              <Logo />
            </Link>
          </div>
        </div>
      </nav>
    </>
  );
};

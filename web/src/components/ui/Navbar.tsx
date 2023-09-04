import Link from "next/link";
import { FC } from "react";
import { Logo } from "./Logo";

export const NavBar: FC = () => {
  return (
    <>
      <nav className="bg-white border-b border-gray-200">
        <div className="max-w-screen flex flex-wrap items-center justify-between">
          <div className="max-w-screen-xl flex flex-wrap items-center justify-between p-2">
            <Link href="/home">
              <Logo />
            </Link>
          </div>
          <ul className="font-medium flex space-x-8 mt-0 border-0 bg-white p-4">
            <div className="bg-blue-500 text-white rounded-md px-2 py-1">
              <span>Sign in</span>
            </div>
            <div className="rounded-md px-2 py-1">
              <span>Sign up</span>
            </div>
          </ul>
        </div>
      </nav>
    </>
  );
};

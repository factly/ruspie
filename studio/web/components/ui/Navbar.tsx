"use client";
import Link from "next/link";
import { FC } from "react";
import { Logo } from "./Logo";
import { Button } from "./Button";
import { kratosUrl } from "@/lib/utils/kratosUrl";
import axios from "axios";
import { kavachEnabled } from "@/lib/utils/kavachAxiosConfig";
import { toast } from "react-hot-toast";
import { useCookies } from "next-client-cookies";
export const NavBar: FC = () => {
  const cookies = useCookies();
  const logout = async () => {
    try {
      const res = await axios(kratosUrl + "/self-service/logout/browser", {
        withCredentials: true,
      });
      console.log(res);
      console.log(cookies.remove("ory_kratos_session"));
      cookies.remove("ory_kratos_session");
    } catch (err) {
      console.log(err);
      toast.error("Something went wrong");
    }
  };
  return (
    <>
      <nav className="bg-white border-b border-gray-200 sticky top-0 w-full">
        <div className="my-3 max-w-screen flex flex-wrap items-center justify-between">
          <div className="max-w-screen-xl flex items-center justify-between p-2">
            <Link href={"/home/organisations"}>
              <Logo />
            </Link>
          </div>
          {/* {kavachEnabled ? ( */}
          {/*   <Button */}
          {/*     onClick={() => logout()} */}
          {/*     className="bg-red-500 mx-2 text-white" */}
          {/*   > */}
          {/*     Logout */}
          {/*   </Button> */}
          {/* ) : null} */}
        </div>
      </nav>
    </>
  );
};

import { FC } from "react";
import { NavBar } from "../../components/ui/Navbar";

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: FC<LayoutProps> = ({ children }) => {
  return (
    <div className="w-screen flex flex-col">
      <div className="fixed w-full z-10">
        <NavBar />
      </div>
      <div className="mt-20">{children}</div>
    </div>
  );
};

export default Layout;

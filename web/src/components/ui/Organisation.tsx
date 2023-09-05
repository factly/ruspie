import { Organisation as Org } from "@/types/organisation";
import { FC } from "react";
import Avatar from "./Avatar";
import { Button } from "./button";
import Icons from "../icons";

interface OrganisationProps {
  org: Org;
  isOpen: Boolean;
  setIsOpen: () => void;
}



export const Organisation: FC<OrganisationProps> = ({ org, isOpen, setIsOpen }) => {
  const handleEditClick = () => {
    console.log("edit clicked");
  };

  const handleDeleteClick = () => {
    console.log("delete clicked");
  };

  const handleProjectClick = () => {
    console.log("project clicked");
  };


  return (
    <div
      className="flex flex-col bg-[#EFF5F9] items-center p-4 w-full rounded-md gap-4 cursor-pointer"
      onClick={setIsOpen}
    >
      <div className="flex flex-row justify-between w-full">
        <div className="flex gap-2 items-center">
          <Avatar src={org.logo || ""} alt={`logo of ${org.title}`} />
          <h2 className="text-lg">{org.title}</h2>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="icon" className="rounded border border-[#E6E6E6]"
            onClick={handleEditClick}
          >
            <Icons.EditIcon />
          </Button>
          <Button variant="outline" size="icon" className="rounded border border-[#E6E6E6]"
            onClick={handleDeleteClick}
          >
            <Icons.TrashIcon />
          </Button>
        </div>
      </div>
      <div className={`flex flex-col gap-2 w-full transition-all duration-300 ${isOpen ? 'h-full' : 'h-0 overflow-hidden hidden'}`}>
        {
          org.projects?.map((project) => (
            <div
              className="flex flex-row justify-between items-center bg-white w-full p-4"
              key={org.id + "_" + project.id + "_" + project.title}
            >
              <div className="flex flex-col gap-2">
                <h3 className="text-md">
                  {project.title}
                </h3>
                <p className="text-sm text-[#6B7280]">
                  Created at: {" "}
                  <span className="text-[#6B7280] font-semibold">
                    {project.createdAt}
                  </span>
                </p>
              </div>
              <Button
                variant="outline"
                size="icon"
                className="rounded border border-[#E6E6E6]"
                onClick={handleProjectClick}
              >
                <Icons.ChevronRightIcon />
              </Button>
            </div>
          ))
        }
        <Button className='rounded-md py-3 px-5 bg-[#666666] text-white' variant='secondary' onClick={handleProjectClick}>
          View All Projects
        </Button>
      </div>
    </div>
  )
};

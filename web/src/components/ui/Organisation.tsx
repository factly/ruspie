import { Organisation as Org } from "@/types/organisation";
import { FC } from "react";
import Avatar from "./Avatar";
import { Button } from "./button";
import Icons from "../icons";

export const Organisation: FC<{ org: Org }> = ({ org }) => {
  const handleEditClick = () => {
    console.log("edit clicked");
  };

  const handleDeleteClick = () => {
    console.log("delete clicked");
  };

  return (
    <div className="flex flex-col bg-[#EFF5F9] items-center p-4 w-full rounded-md">
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
      {
        org.projects?.map((project) => (
          <div className="flex flex-row justify-between bg-white w-full" key={project.id + "_" + project.title}>
            <div className="flex flex-col gap-2">
              <h3 className="text-md">
                {project.title}
              </h3>
              <p className="text-sm text-[#6B7280]">
                Created at:
                <span className="text-[#6B7280] font-semibold">
                  {project.createdAt}
                </span>
              </p>
            </div>
          </div>
        ))
      }
    </div>
  )
};

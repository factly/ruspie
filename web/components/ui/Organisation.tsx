import { Organisation as Org } from "@/types/organisation";
import { FC } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "./Avatar";
import { Button } from "./Button";
import Icons from "../icons";
import Link from "next/link";
import DeleteButttonWithConfirmModal from "./DeleteButttonWithConfimModal";
import axios from "axios";
import { toast } from "react-hot-toast";
import { formatTimestamp } from "@/lib/utils/formatDate";
import { useOrganisationsStore } from "@/lib/zustand/organisation";

interface OrganisationProps {
  org: Org;
  isOpen: Boolean;
  setIsOpen: () => void;
}

export const Organisation: FC<OrganisationProps> = ({
  org,
  isOpen,
  setIsOpen,
}) => {
  const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
    event.stopPropagation();
  };

  const handleDelete = async () => {
    console.log("clicked");
    try {
      const res = await axios.delete(`/api/organisations/${org.id}`);
      toast.success(res.data);
    } catch (err) {
      console.log(err);
      toast.error("Something went wrong");
    }
  };

  const { organisations, setOrganisations } = useOrganisationsStore();
  return (
    <div
      className="flex flex-col bg-[#EFF5F9] items-center p-4 w-full rounded-md gap-4 cursor-pointer"
      onClick={setIsOpen}
    >
      <div className="flex flex-row justify-between w-full">
        <div className="flex gap-2 items-center">
          <Avatar>
            <AvatarImage src={org.logo} alt={`logo of organisation`} />
            <AvatarFallback>
              <Icons.DefaultOrganisation />
            </AvatarFallback>
          </Avatar>
          <h2 className="text-lg">{org.title}</h2>
        </div>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="icon"
            className="rounded border border-[#E6E6E6]"
          >
            <Link href={`/home/organisations/${org.id}/edit`}>
              <Icons.EditIcon />
            </Link>
          </Button>
          <DeleteButttonWithConfirmModal
            onButtonClick={handleDeleteClick}
            onConfirm={async () => {
              await handleDelete();
            }}
            onCancel={() => { }}
            onConfirmFinish={() => {
              const newOrgs = organisations.filter((o) => org.id !== o.id);
              setOrganisations(newOrgs);
            }}
            id={org.id}
          />
        </div>
      </div>
      <div
        className={`flex flex-col gap-2 w-full transition-all duration-300 ${isOpen ? "h-full" : "h-0 overflow-hidden hidden"
          }`}
      >
        {org.projects?.map((project) => (
          <div
            className="flex flex-row justify-between items-center bg-white w-full p-4 cursor-default"
            key={org.id + "_" + project.id + "_" + project.title}
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            <div className="flex flex-col gap-2">
              <h3 className="text-md">{project.title}</h3>
              <p className="text-sm text-[#6B7280]">
                Created at:{" "}
                <span className="text-[#6B7280] font-semibold text-xs">
                  {formatTimestamp(project.createdAt)}
                </span>
              </p>
            </div>
            <Button
              variant="outline"
              size="icon"
              className="rounded border border-[#E6E6E6]"
              asChild
            >
              <Link
                href={`/home/organisations/${org.id}/projects/${project.id}`}
              >
                <Icons.ChevronRightIcon />
              </Link>
            </Button>
          </div>
        ))}
        <Button
          className="rounded-md py-3 px-5 bg-[#666666] text-white"
          variant="secondary"
          asChild
        >
          <Link
            href={
              `/home/organisations/${org.id}` +
              (org.projects?.length === 0 ? "/projects/new" : "")
            }
            className="w-full"
          >
            {org.projects?.length === 0 ? "Add a project" : "View all projects"}
          </Link>
        </Button>
      </div>
    </div>
  );
};

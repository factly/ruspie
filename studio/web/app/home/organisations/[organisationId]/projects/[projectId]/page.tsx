"use client";
import Icons from "@/components/icons";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/Avatar";
import { Button } from "@/components/ui/Button";
import Link from "next/link";
import DatasetTable from "@/components/ui/DatasetTable";
import { useEffect, useState } from "react";
import { Organisation, Project } from "@/types/organisation";
import { Loader } from "lucide-react";
import { toast } from "react-hot-toast";
import axios, { AxiosResponse } from "axios";
import { ProjectParam } from "@/types/params/project_param";
import { useFileStore } from "@/lib/zustand/files";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { kavachAxiosConfig } from "@/lib/utils/kavachAxiosConfig";

export default function Page({
  params: { organisationId, projectId },
}: ProjectParam) {
  const [org, setOrg] = useState<Organisation>();
  const [project, setProject] = useState<Project>();
  const [loading, setLoading] = useState<boolean>();
  const { files, setFiles } = useFileStore();
  const serverUrl = getServerUrl();

  useEffect(() => {
    const fetchProject = async () => {
      setLoading(true);
      try {
        const res: AxiosResponse = await axios.get(
          serverUrl + `/organisations/${organisationId}/projects/${projectId}`,
          {
            ...kavachAxiosConfig,
          },
        );
        setOrg(res.data.organisation);
        setProject(res.data);
        setFiles(res.data.files);
      } catch (err) {
        toast.error("Something went wrong");
      } finally {
        setLoading(false);
      }
    };
    fetchProject();
  }, []);

  if (loading && !project) {
    return (
      <div className="h-screen flex items-center justify-center -mt-28">
        <Loader className="h-10 w-10 animate-spin text-gray-400" />
      </div>
    );
  }
  return (
    <main className="flex flex-col mt-10 bg-transparent px-8">
      <div className="flex flex-row justify-between items-start">
        <div className="flex flex-row gap-3 items-center">
          <Avatar>
            <AvatarImage src={org?.logo} alt={`logo of organisation`} />
            <AvatarFallback>
              <Icons.DefaultOrganisation />
            </AvatarFallback>
          </Avatar>
          <Link href={`/home/organisations/${org?.id}`}>
            <h1 className="text-xl font-semibold text-gray-600 hover:text-black cursor-pointer">
              {" "}
              {org?.title}{" "}
            </h1>
          </Link>
          {">"}
          <h1 className="text-xl font-semibold">{project?.title}</h1>
        </div>
        <Button className="rounded-md bg-[#376789] text-white gap-2" asChild>
          <Link
            href={`/home/organisations/${org?.id}/projects/${project?.id}/dataset/new`}
          >
            <Icons.PlusIcon /> New Dataset
          </Link>
        </Button>
      </div>
      {files?.length === 0 ? (
        <div className="flex flex-col items-center gap-4 my-auto w-full">
          <Icons.NotFound />
          <p className="text-xl w-fit font-medium">
            Oops! nothing found. Get started by creating new Dataset
          </p>
        </div>
      ) : org && project ? (
        <DatasetTable orgId={org.id} projectId={project.id} />
      ) : (
        <div className="h-screen flex items-center justify-center -mt-28">
          <Loader className="h-10 w-10 animate-spin text-gray-400" />
        </div>
      )}
    </main>
  );
}

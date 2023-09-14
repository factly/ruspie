"use client";
import React, { useEffect } from "react";
import Icons from "@/components/icons";
import { Button } from "@/components/ui/Button";
import Projects from "@/components/ui/Projects";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/Avatar";
import Link from "next/link";
import { Organisation } from "@/types/organisation";
import axios, { AxiosResponse } from "axios";
import { toast } from "react-hot-toast";
import { Loader } from "lucide-react";
import { OrgaisationParam } from "@/types/params/oragnisation_param";
import { useProjectsStore } from "@/lib/zustand/projects";

export default function Page({ params: { organisationId } }: OrgaisationParam) {
  const [organisation, setOrganisation] = React.useState<Organisation | null>(
    null,
  );
  const { projects, setProjects } = useProjectsStore();

  async function fetchOrganisation() {
    setLoading(true);
    try {
      const res: AxiosResponse<Organisation> = await axios(
        "/api/organisations/" + organisationId,
      );
      setOrganisation(res.data);
      if (res.data.projects) {
        setProjects(res.data.projects);
      }
    } catch (err) {
      toast.error("Error getting organisation");
    } finally {
      setLoading(false);
    }
  }

  const [loading, setLoading] = React.useState(true);
  useEffect(() => {
    fetchOrganisation();
  }, []);

  if (loading && !organisation) {
    return (
      <div className="h-screen flex items-center justify-center -mt-28">
        <Loader className="h-10 w-10 animate-spin text-gray-400" />
      </div>
    );
  }

  return (
    <main className="flex flex-col mt-10 bg-transparent">
      <div className="flex flex-row justify-around items-start">
        <div className="flex flex-row gap-3 items-center">
          <Avatar>
            <AvatarImage
              src={organisation?.logo}
              alt={`logo of organisation`}
            />
            <AvatarFallback>
              <Icons.DefaultOrganisation />
            </AvatarFallback>
          </Avatar>
          <Link href={`/home/organisations`}>
            <h1 className="text-xl font-semibold"> {organisation?.title} </h1>
          </Link>
        </div>
        <div className="flex flex-col w-2/5 justify-around gap-10">
          {projects.length !== 0 && <Projects org={organisation || null} />}
        </div>
        <Button className="rounded-md bg-[#376789] text-white" asChild>
          <Link href={`/home/organisations/${organisation?.id}/projects/new`}>
            <Icons.PlusIcon /> Add Projects
          </Link>
        </Button>
      </div>
      {projects.length === 0 && (
        <div className="flex flex-col items-center gap-4 my-auto w-full">
          <Icons.NotFound />
          <p className="text-xl w-fit font-medium">
            Oops! nothing found. Get started by creating new Project
          </p>
        </div>
      )}
    </main>
  );
}

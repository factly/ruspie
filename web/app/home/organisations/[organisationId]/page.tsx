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
import { SearchBar } from "@/components/ui/searchBar";
import { getServerUrl } from "@/lib/utils/serverUrl";

export default function Page({ params: { organisationId } }: OrgaisationParam) {
  const [organisation, setOrganisation] = React.useState<Organisation | null>(
    null,
  );
  const { projects, setProjects } = useProjectsStore();
  const serverUrl = getServerUrl();

  async function fetchOrganisation() {
    setLoading(true);
    try {
      const res: AxiosResponse<Organisation> = await axios(
        serverUrl + `/organisations/${organisationId}/`,
        {
          headers: process.env.NEXT_PUBLIC_KAVACH_ENABLED
            ? { "X-User": "1" }
            : undefined,
          withCredentials: true,
        },
      );
      setOrganisation(res.data);
      if (res.data.projects) {
        setProjects(res.data.projects);
      }
    } catch (err) {
      console.log(err);
      toast.error("Error getting projexts");
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
  const handleFilterProject = async (query: string) => {
    setLoading(true);
    try {
      console.log("here");
      const res: AxiosResponse = await axios(
        serverUrl +
        "/organisations/" +
        organisationId +
        "/projects" +
        `?search_query=${query}`,
        {
          headers: process.env.NEXT_PUBLIC_KAVACH_ENABLED
            ? { "X-User": "1" }
            : undefined,
          withCredentials: true,
        },
      );
      setProjects(res.data.projects);
    } catch (err) {
      toast.error("Error getting organisation");
    } finally {
      setLoading(false);
    }
  };
  return (
    <main className="flex flex-col mt-10 bg-transparent px-20">
      <div className="flex flex-row justify-between items-start">
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
            <h1 className="text-xl font-semibold">
              {/* truncate the text if more than 15 char */}
              {organisation?.title && organisation?.title.length > 12
                ? organisation?.title.substring(0, 12) + "..."
                : organisation?.title}
            </h1>
          </Link>
        </div>
        <div className="flex flex-col w-2/5 justify-around gap-10">
          <SearchBar
            placeholder="Search Organisation"
            callback={handleFilterProject}
          />
          {loading ? (
            <div className="h-screen flex items-center justify-center -mt-28">
              <Loader className="h-10 w-10 animate-spin text-gray-400" />
            </div>
          ) : (
            projects.length !== 0 && <Projects orgId={organisationId} />
          )}
        </div>
        <Button className="rounded-md bg-[#376789] text-white ml-10" asChild>
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

"use client";
import React, { useEffect, useState } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import Link from "next/link";
import { useForm } from "react-hook-form";
import { useOrganisationsStore } from "@/lib/zustand/organisation";
import { Organisation, Project } from "@/types/organisation";
import axios, { AxiosError, AxiosResponse } from "axios";
import toast from "react-hot-toast";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  CreateProjectSchema,
  UpdateProjectSchema,
  createProjectSchema,
} from "@/lib/zod/validators/projects";
import { useRouter } from "next/navigation";
import { ProjectParam } from "@/types/params/project_param";
import { Loader } from "lucide-react";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { kavachAxiosConfig } from "@/lib/utils/kavachAxiosConfig";

export default function Page({
  params: { organisationId: orgId, projectId },
}: ProjectParam) {
  const router = useRouter();
  const {
    handleSubmit,
    register,
    formState: { errors },
    setValue,
    watch,
  } = useForm<CreateProjectSchema>({
    resolver: zodResolver(createProjectSchema),
  });
  const { organisations, setOrganisations } = useOrganisationsStore();
  const [loading, setLoading] = React.useState<boolean>(false);
  let newOrgId = orgId;
  const serverUrl = getServerUrl();

  useEffect(() => {
    async function getOrganisations() {
      try {
        setLoading(true);
        const resp: AxiosResponse<{
          code: number;
          organisations: Organisation[];
        }> = await axios.get(serverUrl + "/organisations", {
          ...kavachAxiosConfig,
        });
        const orgs = resp.data.organisations;
        setOrganisations(orgs);
        const projects = orgs.find((org) => {
          return parseInt(org.id) === parseInt(orgId);
        })?.projects;
        if (projects) {
          const project = projects.find(
            (project) => parseInt(project.id) === parseInt(projectId),
          );
          if (!project) {
            toast.error("Something went wrong");
            return;
          }
          setProject(project);
          setValue("title", project.title);
          setValue("description", project.description);
        } else {
          toast.error("Something went wrong");
        }
      } catch (err) {
        toast.error("Error getting organisations");
      } finally {
        setLoading(false);
      }
    }
    getOrganisations();
  }, []);
  const [project, setProject] = useState<Project>();

  if (!project || loading) {
    return (
      <div className="h-screen flex items-center justify-center -mt-28">
        <Loader className="h-10 w-10 animate-spin text-gray-400" />
      </div>
    );
  }

  const titleValue = watch("title");

  const disableButton = titleValue === undefined || titleValue === "";``

  return (
    <main className="flex flex-col mt-10 bg-transparent">
      <div className="flex flex-row justify-around items-start">
        <h1 className="text-xl font-semibold">Edit {project.title}</h1>
        <form className="flex flex-col items-center w-2/5 mt-20 gap-10">
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="title" className="font-normal">
              Title
            </Label>
            <Input
              type="text"
              id="title"
              placeholder="Enter title here"
              {...register("title")}
            />
            {errors.title && (
              <p className="text-red-500 text-xs italic">
                {errors.title.message}
              </p>
            )}
          </div>
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="title" className="font-normal">
              Description
            </Label>
            <Textarea
              id="description"
              placeholder="Enter description here"
              {...register("description")}
            />
            {errors.description && (
              <p className="text-red-500 text-xs italic">
                {errors.description.message}
              </p>
            )}
          </div>
          <Button
            disabled={disableButton}
            onClick={handleSubmit(async (data) => {
              if (newOrgId !== orgId) {
                try {
                  await axios.patch(
                    serverUrl +
                    `/organisations/${orgId}/projects/${projectId}/change_org`,
                    {
                      new_org_id: newOrgId.toString(),
                    },
                    {
                      ...kavachAxiosConfig,
                    },
                  );
                } catch (err) {
                  toast.error("Something went wrong");
                  return;
                }
              }
              try {
                const payload: UpdateProjectSchema = {};
                if (data.title !== project.title) {
                  payload.title = data.title;
                }
                payload.description = data.description;
                await axios.put(
                  serverUrl + `/organisations/${orgId}/projects/${projectId}/`,
                  payload,
                  {
                    ...kavachAxiosConfig,
                  },
                );
                toast.success("Project updated sucessfully");
                router.push(`/home/organisations/${newOrgId}`);
              } catch (err) {
                if (err instanceof AxiosError) {
                  if (err.response?.status === 409) {
                    toast.error("Title already exists");
                    return;
                  }
                }
                toast.error("Something went wrong");
              }
            })}
            className="rounded-md bg-[#376789] text-white w-full"
          >
            Edit Project
          </Button>
        </form>
        <div className="flex gap-3">
          <Button
            variant="outline"
            className="rounded-md text-[#376789] border-[#376789]"
            asChild
          >
            <Link href={`/home/organisations/${orgId}`}>Cancel</Link>
          </Button>
        </div>
      </div>
    </main>
  );
}

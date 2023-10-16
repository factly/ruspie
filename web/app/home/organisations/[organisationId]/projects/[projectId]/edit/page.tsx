"use client";
import React, { useEffect, useState } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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
import { getBasepathUrl } from "@/lib/utils/baseUrl";

export default function Page({
  params: { organisationId: orgId, projectId },
}: ProjectParam) {
  const router = useRouter();
  const {
    handleSubmit,
    register,
    formState: { errors },
    setValue,
  } = useForm<CreateProjectSchema>({
    resolver: zodResolver(createProjectSchema),
  });
  const { organisations, setOrganisations } = useOrganisationsStore();
  const [loading, setLoading] = React.useState<boolean>(false);
  let newOrgId = orgId;
  const basePath = getBasepathUrl();

  useEffect(() => {
    async function getOrganisations() {
      try {
        setLoading(true);
        const resp: AxiosResponse<{
          code: number;
          organisations: Organisation[];
        }> = await axios.get(basePath + "/api/organisations");
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

  const currentOrganisation = organisations.find(
    (org) => parseInt(org.id) === parseInt(orgId),
  );

  return (
    <main className="flex flex-col mt-10 bg-transparent">
      <div className="flex flex-row justify-around items-start">
        <h1 className="text-xl font-semibold">Edit {project.title}</h1>
        <form className="flex flex-col items-center w-2/5 mt-20 gap-10">
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="organisation" className="font-normal">
              Organisation
            </Label>
            <Select
              onValueChange={(value) => {
                organisations.map((org) => {
                  if (org.title === value) {
                    newOrgId = org.id;
                  }
                });
              }}
            >
              <SelectTrigger className="w-full">
                <SelectValue
                  placeholder={currentOrganisation?.title}
                ></SelectValue>
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  {organisations.map((org) => {
                    return (
                      <SelectItem key={org.id} value={org.title}>
                        {org.title}
                      </SelectItem>
                    );
                  })}
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
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
        </form>
        <div className="flex gap-3">
          <Button
            variant="outline"
            className="rounded-md text-[#376789] border-[#376789]"
            asChild
          >
            <Link href={`/home/organisations/${orgId}`}>Cancel</Link>
          </Button>
          <Button
            onClick={handleSubmit(async (data) => {
              if (newOrgId !== orgId) {
                try {
                  await axios.patch(
                    basePath +
                    `/api/organisations/${orgId}/projects/${projectId}`,
                    {
                      new_org_id: newOrgId,
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
                  basePath +
                  `/api/organisations/${orgId}/projects/${projectId}`,
                  payload,
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
            className="rounded-md bg-[#376789] text-white"
          >
            Edit Project
          </Button>
        </div>
      </div>
    </main>
  );
}

"use client";
import React from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import Icons from "@/components/icons";
import Link from "next/link";
import { ProjectParam } from "@/types/params/project_param";
import { CreateFileSchema, createFileSchema } from "@/lib/zod/validators/files";
import axios, { AxiosError } from "axios";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";

export default function Page({
  params: { projectId, organisationId },
}: ProjectParam) {
  const {
    handleSubmit,
    register,
    formState: { errors },
  } = useForm<CreateFileSchema>({
    resolver: zodResolver(createFileSchema),
  });
  const router = useRouter();
  return (
    <main className="flex flex-col mt-10 bg-transparent">
      <div className="flex flex-row justify-around items-start">
        <h1 className="text-xl font-semibold">Add new dataset </h1>
        <form className="flex flex-col items-center w-2/5 mt-20 gap-10">
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="title" className="font-normal">
              Title
            </Label>
            <Input
              type="text"
              id="title"
              {...register("name")}
              placeholder="Enter the title"
            />
            {errors.name ? (
              <div className="text-red-500 italic  text-xs">
                {errors.name.message}
              </div>
            ) : (
              ""
            )}
          </div>
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="Dataset" className="font-normal">
              Upload Dataset
            </Label>
            {/* <Input type="file" id="title" placeholder="Enter title here" /> */}
            <div
              className="rounded-md w-full cursor-pointer h-[250px] flex flex-col justify-center items-center gap-3"
              style={{
                backgroundImage: `url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='7' ry='7' stroke='%2366666669' stroke-width='3' stroke-dasharray='10 24' stroke-dashoffset='0' stroke-linecap='round'/%3e%3c/svg%3e")`,
              }}
            >
              <Icons.UploadClould />
              <p className="text-sm text-gray-400">
                Drag and drop or Click to upload
              </p>
            </div>
          </div>
        </form>
        <div className="flex gap-3">
          <Button
            variant="outline"
            className="rounded-md text-[#376789] border-[#376789]"
            asChild
          >
            <Link
              href={`/home/organisations/${organisationId}/projects/${projectId}/`}
            >
              Cancel
            </Link>
          </Button>
          <Button
            onClick={handleSubmit(async (data) => {
              const file = {
                name: data.name,
                s3_url: "http://minio:9000/ruspie/lead_shot.csv",
                extension: "csv",
              };
              try {
                await axios.post(
                  `/api/organisations/${organisationId}/projects/${projectId}/datasets`,
                  file,
                );
                toast.success("Successfully created new dataset");
                router.push(
                  `/home/organisations/${organisationId}/projects/${projectId}`,
                );
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
            Add Dataset
          </Button>
        </div>
      </div>
    </main>
  );
}

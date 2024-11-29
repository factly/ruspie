"use client";
import React, { useRef, useState } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import Link from "next/link";
import Image from "next/image";
import { ProjectParam } from "@/types/params/project_param";
import { CreateFileSchema, createFileSchema } from "@/lib/zod/validators/files";
import axios, { AxiosError } from "axios";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import UppyUploader from "@/components/UppyUploader";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { kavachAxiosConfig } from "@/lib/utils/kavachAxiosConfig";
import UploadImage from "@/assets/uploadImage.png";
import FileImage from "@/assets/document.png";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { title } from "process";

export default function Page({
  params: { projectId, organisationId },
}: ProjectParam) {
  const {
    handleSubmit,
    register,
    formState,
    getValues
  } = useForm<CreateFileSchema>({
    resolver: zodResolver(createFileSchema),
  });
  const [title, setTitle] = useState<string>("");
  const [filename, setFilename] = useState<string>("");
  const [uploadDialogOpen, setUploadDialogOpen] = useState<boolean>(false)
  const router = useRouter();
  const {errors} = formState

  const diableAddButton = filename === '' || title === ''
  
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
              onChange={(e) => setTitle(e.target.value)}
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
            <Label htmlFor="title" className="font-normal">
              Upload Dataset
            </Label>
            <Dialog open={uploadDialogOpen} onOpenChange={setUploadDialogOpen}>
              <DialogTrigger asChild>
                <div className="p-6 border border-input rounded-md w-fit cursor-pointer">
                  <Image
                    src={filename !== '' ? FileImage : UploadImage}
                    alt="logo"
                    width={125}
                    height={125}
                  />
                  {filename}
                </div>
              </DialogTrigger>
              <DialogContent className="w-fit max-w-4xl">
                <UppyUploader
                  onUpload={(_, path) => {
                    setFilename(path);
                    setUploadDialogOpen(false);
                  }}
                  isDataset={true}
                />
              </DialogContent>
            </Dialog>
          </div>
          <Button
            disabled={diableAddButton || false}
            onClick={handleSubmit(async (data) => {
              const file = {
                name: data.name.trim(),
                s3_url: "http://minio:9000/ruspie/" + filename,
                extension: filename.split(".")[1],
              };
              try {
                await axios.post(
                  getServerUrl() +
                  `/organisations/${organisationId}/projects/${projectId}/files/`,
                  file,
                  {
                    ...kavachAxiosConfig,
                  },
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
            className="rounded-md bg-[#376789] text-white w-full"
          >
            Add Dataset
          </Button>
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
        </div>
      </div>
    </main>
  );
}

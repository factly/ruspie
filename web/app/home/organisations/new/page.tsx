"use client";
import React from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import Link from "next/link";
import Image from "next/image";
import UploadImage from "@/assets/uploadImage.png";
import { useForm } from "react-hook-form";
import {
  CreateOrganisationSchema,
  createOrganisationSchema,
} from "@/lib/zod/validators/organisation";
import { zodResolver } from "@hookform/resolvers/zod";
import Link from "next/link";
import axios, { AxiosError, AxiosResponse } from "axios";
import { Organisation } from "@/types/organisation";
import { toast } from "react-hot-toast";
import { ZodError } from "zod";
import { useRouter } from "next/navigation";

export default function Page() {
  const {
    handleSubmit,
    formState: { errors },
    register,
    reset,
  } = useForm<CreateOrganisationSchema>({
    resolver: zodResolver(createOrganisationSchema),
  });
  const createOrganisation = async (data: CreateOrganisationSchema) => {
    const res: AxiosResponse<Organisation> = await axios.post(
      "/api/organisations",
      data,
    );
    return res.data;
  };
  const router = useRouter();
  return (
    <main className="flex flex-col mt-10 bg-transparent">
      <div className="flex flex-row justify-around items-start">
        <h1 className="text-xl font-semibold">Add Organization </h1>
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
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="title" className="font-normal">
              Logo
            </Label>
            {/* <Input type="file" id="title" placeholder="Enter title here" /> */}
            <div className="p-6 border border-input rounded-md w-fit cursor-pointer">
              <Image src={UploadImage} alt="logo" width={125} height={125} />
            </div>
          </div>
        </form>
        <div className="flex gap-3">
          <Link href="/home/organisations">
            <Button
              variant="outline"
              className="rounded-md text-[#376789] border-[#376789]"
            >
              Cancel
            </Button>
          </Link>
          <Button
            onClick={handleSubmit(async (data) => {
              try {
                await createOrganisation(data);
                toast.success("Organisation created successfully");
                reset();
                router.push("/home/organisations");
              } catch (err) {
                if (err instanceof AxiosError) {
                  const { response } = err;
                  if (response?.status === 409 || response?.status === 400) {
                    toast.error(response?.data.message);
                  }
                  return;
                }
                if (err instanceof ZodError) {
                  toast.error(err.message);
                  return;
                }
                toast.error("Something went wrong");
              }
            })}
            className="rounded-md bg-[#376789] text-white"
          >
            Create Organization
          </Button>
        </div>
      </div>
    </main>
  );
}

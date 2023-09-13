"use client";
import React, { useEffect } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import Image from "next/image";
import UploadImage from "@/assets/uploadImage.png";
import { useForm } from "react-hook-form";
import {
  UpdateOrganisationSchema,
  updateOrganisationSchema,
} from "@/lib/zod/validators/organisation";
import { zodResolver } from "@hookform/resolvers/zod";
import Link from "next/link";
import axios, { AxiosError, AxiosResponse } from "axios";
import { Organisation } from "@/types/organisation";
import { toast } from "react-hot-toast";
import { ZodError } from "zod";
import { useRouter } from "next/navigation";
import { Loader } from "lucide-react";
import { OrgaisationParam } from "@/types/params/oragnisation_param";

export default function Page({ params: { organisationId } }: OrgaisationParam) {
  const [organisation, setOrganisation] = React.useState<Organisation | null>(
    null,
  );
  const [loading, setLoading] = React.useState<boolean>(false);
  useEffect(() => {
    async function fetchOrganisation() {
      setLoading(true);
      try {
        const res: AxiosResponse<Organisation> = await axios(
          "/api/organisations/" + organisationId,
        );
        if (!res.data.projects) {
          res.data.projects = [
            {
              id: "1",
              createdAt: "2021-08-23T18:25:43.511Z",
              updatedAt: "2021-08-23T18:25:43.511Z",
              title: "Project 1",
            },
          ];
        }
        setOrganisation(res.data);
        setValue("title", res.data.title);
        console.log(res.data);
        setValue("description", res.data.description);
      } catch (err) {
        toast.error("Somethign went wrong");
      } finally {
        setLoading(false);
      }
    }
    fetchOrganisation();
  }, []);

  const {
    handleSubmit,
    formState: { errors },
    register,
    reset,
    setValue,
  } = useForm<UpdateOrganisationSchema>({
    resolver: zodResolver(updateOrganisationSchema),
  });
  const updateOrganisation = async (data: UpdateOrganisationSchema) => {
    let payload: UpdateOrganisationSchema = {};
    if (data.title !== organisation?.title) {
      payload.title = data.title;
    }
    payload.description = data.description;
    payload.logo = data.description;
    const res: AxiosResponse<Organisation> = await axios.put(
      `/api/organisations/${organisationId}`,
      payload,
    );
    return res.data;
  };
  const router = useRouter();

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
        <h1 className="text-xl font-semibold">Edit {organisation?.title}</h1>
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
                await updateOrganisation(data);
                toast.success("Organisation edited successfully");
                reset();
                router.push("/home/organisations");
              } catch (err) {
                if (err instanceof AxiosError) {
                  const { response } = err;
                  if (response?.status === 409) {
                    toast.error("Organisation title already exists");
                    return;
                  }
                  toast.error("Something went wrong");
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
            Edit Organisation
          </Button>
        </div>
      </div>
    </main>
  );
}

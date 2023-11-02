"use client";
import React, { useEffect } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import Link from "next/link";
import { useForm } from "react-hook-form";
import { useOrganisationsStore } from "@/lib/zustand/organisation";
import { Organisation } from "@/types/organisation";
import axios, { AxiosError, AxiosResponse } from "axios";
import toast from "react-hot-toast";
import { zodResolver } from "@hookform/resolvers/zod";
import {
	CreateProjectSchema,
	createProjectSchema,
} from "@/lib/zod/validators/projects";
import { useRouter } from "next/navigation";
import { OrgaisationParam } from "@/types/params/oragnisation_param";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { kavachAxiosConfig } from "@/lib/utils/kavachAxiosConfig";

export default function Page({
	params: { organisationId: orgId },
}: OrgaisationParam) {
	const router = useRouter();
	const serverUrl = getServerUrl();

	const {
		handleSubmit,
		register,
		formState: { errors },
		watch
	} = useForm<CreateProjectSchema>({
		resolver: zodResolver(createProjectSchema),
	});

	const { organisations, setOrganisations } = useOrganisationsStore();
	useEffect(() => {
		async function getOrganisations() {
			try {
				const resp: AxiosResponse<{
					code: number;
					organisations: Organisation[];
				}> = await axios.get(serverUrl + "/organisations", {
					...kavachAxiosConfig,
				});
				setOrganisations(resp.data.organisations);
			} catch (err) {
				toast.error("Error getting organisations");
			}
		}
		getOrganisations();
	}, []);

	const watchTitle = watch("title");
	const disableButton = watchTitle === undefined || watchTitle === "" || orgId === undefined || orgId === "";

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<h1 className="text-xl font-semibold">Create New Project </h1>
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
							try {
								await axios.post(
									serverUrl + `/organisations/${orgId}/projects`,
									data,
									{
										...kavachAxiosConfig,
									},
								);
								toast.success("Project created sucessfully");
								router.push(`/home/organisations/${orgId}`);
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
						Create Project
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

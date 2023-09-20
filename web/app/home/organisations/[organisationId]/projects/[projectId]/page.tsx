"use client";
import Icons from "@/components/icons";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/Avatar";
import { Button } from "@/components/ui/Button";
import Link from "next/link";
import DatasetTable from "@/components/ui/DatasetTable";
import { File as Dataset } from "@/types/file";
import { useState } from "react";
import { Organisation, Project } from "@/types/organisation";
import { Loader } from "lucide-react";

export default function Page() {
	const [org, setOrg] = useState<Organisation>();
	const [project, setProject] = useState<Project>();
	const [loading, setLoading] = useState<boolean>();
	const [datasets, setDatasets] = useState<Dataset[]>();

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
			{datasets?.length === 0 ? (
				<div className="flex flex-col items-center gap-4 my-auto w-full">
					<Icons.NotFound />
					<p className="text-xl w-fit font-medium">
						Oops! nothing found. Get started by creating new Dataset
					</p>
				</div>
			) : (
				<DatasetTable org={org} project={project} datasets={datasets || []} />
			)}
		</main>
	);
}

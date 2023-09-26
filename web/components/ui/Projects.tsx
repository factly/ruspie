"use client";
import React, { FC } from "react";
import { Button } from "./Button";
import Icons from "../icons";
import { SearchBar } from "./searchBar";
import DeleteButttonWithConfirmModal from "./DeleteButttonWithConfimModal";
import Link from "next/link";
import { formatTimestamp } from "@/lib/utils/formatDate";
import axios from "axios";
import toast from "react-hot-toast";
import { useProjectsStore } from "@/lib/zustand/projects";
import { useRouter } from "next/navigation";

interface OrganisationProps {
	orgId: string;
}

const Project: FC<OrganisationProps> = ({ orgId }) => {
	const router = useRouter();
	const handleFilterProject = (query: string) => {
		console.log(query);
	};

	const handleEditClick = (projectId: string, orgId: string) => {
		router.push(`/home/organisations/${orgId}/projects/${projectId}/edit`);
	};

	const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
		event.stopPropagation();
		event.nativeEvent.preventDefault();
	};

	const handleDelete = async (projectId: string, orgId: string) => {
		try {
			const res = await axios.delete(
				`/api/organisations/${orgId}/projects/${projectId}`,
			);
			toast.success(res.data);
		} catch (err) {
			console.log(err);
			toast.error("Something went wrong");
		}
	};
	const { projects, setProjects } = useProjectsStore();
	return (
		<>
			<SearchBar
				placeholder="Search Organisation"
				callback={handleFilterProject}
			/>
			<div className="flex flex-col items-center  max-h-[60vh] overflow-y-auto">
				{projects?.map((project, index) => (
					<div
						className={`flex flex-row justify-between items-center bg-white w-full p-4 border-b border-[#EAECF0]
								${index === 0 ? "rounded-t-md" : ""}
								${projects !== undefined && index === projects.length - 1 ? "rounded-b-md" : ""}
							`}
						key={orgId + "_" + project.id + "_" + project.title}
					>
						<Link
							href={`/home/organisations/${orgId}/projects/${project.id}`}
							className="flex flex-col gap-2"
						>
							<h3 className="text-md">{project.title}</h3>
							<p className="text-sm text-[#6B7280]">
								Created at:{" "}
								<span className="text-[#6B7280] font-semibold text-xs">
									{formatTimestamp(project.createdAt)}
								</span>
							</p>
						</Link>
						<div className="flex gap-2">
							<Button
								variant="outline"
								size="icon"
								className="rounded border border-[#E6E6E6]"
								onClick={() => handleEditClick(project.id, orgId)}
							>
								<Icons.EditIcon />
							</Button>
							<DeleteButttonWithConfirmModal
								onButtonClick={handleDeleteClick}
								onConfirm={async () => {
									await handleDelete(project.id, orgId);
								}}
								onConfirmFinish={() => {
									const newPros = projects.filter(
										(pro) => pro.id !== project.id,
									);
									setProjects(newPros || []);
								}}
								onCancel={() => { }}
							/>
						</div>
					</div>
				))}
			</div>
		</>
	);
};

export default Project;

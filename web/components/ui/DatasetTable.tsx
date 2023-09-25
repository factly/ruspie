"use client";
import Icons from "@/components/icons";
import { Button } from "@/components/ui/Button";
import Link from "next/link";
import React, { FC } from "react";
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import DeleteButttonWithConfirmModal from "./DeleteButttonWithConfimModal";
import { formatTimestamp } from "@/lib/utils/formatDate";
import axios from "axios";
import toast from "react-hot-toast";
import { useFileStore } from "@/lib/zustand/files";

interface DatasetTableProps {
	orgId: string;
	projectId: string;
}

const DatasetTable: FC<DatasetTableProps> = ({ orgId, projectId }) => {
	const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
		event.stopPropagation();
	};

	const { files, setFiles } = useFileStore();
	const handleDelete = async (datasetId: string) => {
		try {
			const res = await axios.delete(
				`/api/organisations/${orgId}/projects/${projectId}/datasets/${datasetId}`,
			);
			toast.success(res.data);
		} catch (err) {
			console.log(err);
			toast.error("Something went wrong");
		}
	};

	return (
		<Table className="mx-auto mt-10 w-9/10 md:w-4/5">
			<TableHeader>
				<TableRow className="bg-transparent text-[#666666] border-t rounded-t-md border-x">
					<TableHead className="p-4">Dataset Name</TableHead>
					<TableHead className="p-4">Created at</TableHead>
					<TableHead className="p-4">Updated at</TableHead>
					<TableHead className="p-4 text-center">Actions</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody className="bg-white">
				{files.map((dataset) => (
					<TableRow key={dataset.id}>
						<TableCell className="p-4">{dataset.name}</TableCell>
						<TableCell className="p-4 text-[#1e1e1e]">
							{formatTimestamp(dataset.createdAt.toString())}
						</TableCell>
						<TableCell className="p-4 text-[#1e1e1e]">
							{formatTimestamp(dataset.updatedAt.toString())}
						</TableCell>
						<TableCell className="p-4">
							<div className="flex flex-row gap-2 justify-center">
								<Button
									variant="outline"
									size="icon"
									className="rounded border border-[#E6E6E6]"
									asChild
								>
									<Link
										href={`/home/organisations/${orgId}/projects/${projectId}/dataset/1`}
									>
										<Icons.ArrowRight />
									</Link>
								</Button>
								<DeleteButttonWithConfirmModal
									onButtonClick={handleDeleteClick}
									onConfirm={async () => {
										await handleDelete(dataset.id);
									}}
									onConfirmFinish={() => {
										const newFiles = files.filter(
											(file) => file.id != dataset.id,
										);
										setFiles(newFiles);
									}}
									onCancel={() => { }}
								/>
							</div>
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	);
};

export default DatasetTable;

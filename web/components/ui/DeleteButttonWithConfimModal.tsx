"use client";
import React from "react";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/Button";
import Icons from "@/components/icons";
import { Loader } from "lucide-react";
import { useOrganisationsStore } from "@/lib/zustand/organisation";

interface DeleteButtonWithConfirmModalProps {
	onConfirm: () => Promise<void>;
	onButtonClick: (e: any) => void;
	onCancel: () => void;
	onConfirmFinish?: () => void;
	id?: string;
}
function DeleteButttonWithConfirmModal(
	props: DeleteButtonWithConfirmModalProps,
) {
	const [open, setOpen] = React.useState(false);
	const [loading, setLoading] = React.useState(false);

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger asChild>
				<Button
					variant="outline"
					size="icon"
					className="rounded border border-[#E6E6E6]"
					onClick={props.onButtonClick}
				>
					<Icons.TrashIcon />
				</Button>
			</DialogTrigger>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Are you sure absolutely sure?</DialogTitle>
					<DialogDescription className="text-sm !mt-4 text-red-500 bg-red-200 p-2 rounded">
						This action cannot be undone. This will permanently delete your
						account and remove your data from our servers.
					</DialogDescription>
				</DialogHeader>
				<DialogFooter>
					<Button
						variant="ghost"
						onClick={() => {
							setOpen(false);
							props.onCancel();
						}}
					>
						Cancel
					</Button>
					<Button
						variant="destructive"
						onClick={async () => {
							try {
								setLoading(true);
								await props.onConfirm();
							} finally {
								setOpen(false);
								setLoading(false);
								// const newOrgs = organisations.filter(
								// 	(org) => org.id !== props.id,
								// );
								// setOrganisations(newOrgs);
								if (props.onConfirmFinish) {
									props.onConfirmFinish();
								}
							}
						}}
					>
						{loading ? (
							<Loader className="h-3 w-3 animate-spin mr-2 text-white" />
						) : (
							""
						)}
						Delete
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}

export default DeleteButttonWithConfirmModal;

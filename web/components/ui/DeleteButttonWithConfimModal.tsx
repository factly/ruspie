'use client';
import React from "react"
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/Button"
import Icons from "@/components/icons"


function DeleteButttonWithConfimModal(props: any) {
	const [open, setOpen] = React.useState(false)

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
						This action cannot be undone. This will permanently delete your account
						and remove your data from our servers.
					</DialogDescription>
				</DialogHeader>
				<DialogFooter>
					<Button variant="ghost"
						onClick={() => {
							setOpen(false)
							props.onCancel()
						}}>
						Cancel
					</Button>
					<Button variant="destructive" onClick={props.onConfirm}>
						Delete
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	)
}

export default DeleteButttonWithConfimModal

"use client";
import React, { useEffect } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function Page() {
	const pathname = usePathname();
	const orgId = pathname.split("/")[3];

	const [formData, setFormData] = React.useState({
		title: "",
		description: "",
		logo: "",
	});

	type SelectOption = {
		value: string;
		name: string;
	};

	const handleChange = (
		e:
			| React.ChangeEvent<HTMLInputElement>
			| React.ChangeEvent<HTMLTextAreaElement>
			| SelectOption,
	) => {
		if (typeof e === "object" && "value" in e) {
			setFormData({
				...formData,
				[e.name]: e.value,
			});
			return;
		}

		const { name, value } = e.target;
		setFormData({
			...formData,
			[name]: value,
		});
		console.log(formData);
	};

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<h1 className="text-xl font-semibold">Create New Project </h1>
				<form className="flex flex-col items-center w-2/5 mt-20 gap-10">
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="title" className="font-normal">
							Title
						</Label>
						<Select
							onValueChange={(value) =>
								handleChange({
									name: "organisation",
									value,
								})
							}
						>
							<SelectTrigger className="w-full">
								<SelectValue placeholder="Select a Organisation" />
							</SelectTrigger>
							<SelectContent>
								<SelectGroup>
									<SelectLabel>Organisations</SelectLabel>
									<SelectItem value="new">
										TODO: Render organisations
									</SelectItem>
									<SelectItem value="new2">
										TODO: Render organisations
									</SelectItem>
								</SelectGroup>
							</SelectContent>
						</Select>
					</div>
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="title" className="font-normal">
							Title
						</Label>
						<Input
							name="title"
							type="text"
							id="title"
							placeholder="Enter title here"
							onChange={handleChange}
							value={formData.title}
						/>
					</div>
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="title" className="font-normal">
							Description
						</Label>
						<Textarea
							name="description"
							id="description"
							placeholder="Enter description here"
							onChange={handleChange}
							value={formData.description}
						/>
					</div>
				</form>
				<div className="flex gap-3">
				<Button
						variant="outline"
						className="rounded-md text-[#376789] border-[#376789]"
						asChild
					>
						<Link href={`/home/organisations/${orgId}`}>Cancel</Link>
					</Button>
					<Button className="rounded-md bg-[#376789] text-white">
						Create Project
					</Button>
				</div>
			</div>
		</main>
	);
}

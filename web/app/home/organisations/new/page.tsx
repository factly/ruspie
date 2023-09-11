"use client";
import React, { useEffect } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import { Textarea } from "@/components/dataEntry/textarea";
import Link from "next/link";
import Image from "next/image";
import UploadImage from "@/assets/uploadImage.png";

export default function Page() {
	const [formData, setFormData] = React.useState({
		title: "",
		description: "",
		logo: "",
	});

	const handleChange = (
		e:
			| React.ChangeEvent<HTMLInputElement>
			| React.ChangeEvent<HTMLTextAreaElement>,
	) => {
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
				<h1 className="text-xl font-semibold">Add Organization </h1>
				<form className="flex flex-col items-center w-2/5 mt-20 gap-10">
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
				<Button
						variant="outline"
						className="rounded-md text-[#376789] border-[#376789]"
						asChild
					>
						<Link href={`/home/organisations/`}>Cancel</Link>
					</Button>
					<Button className="rounded-md bg-[#376789] text-white">
						Create Organization
					</Button>
				</div>
			</div>
		</main>
	);
}

;

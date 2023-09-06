"use client";
import React, { useEffect } from "react";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/dataEntry/input";
import { Label } from "@/components/dataEntry/label";
import Icons from "@/components/icons";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function Page() {
	const pathname = usePathname()
	const orgId = pathname.split('/')[3]
	const projectId = pathname.split('/')[5]

	const [formData, setFormData] = React.useState({
		title: "",
		description: "",
		logo: "",
	});

	const handleChange = (
		e:
			| React.ChangeEvent<HTMLInputElement>
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
				<h1 className="text-xl font-semibold">Add new dataset </h1>
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
						<Label htmlFor="Dataset" className="font-normal">
							Upload Dataset
						</Label>
						{/* <Input type="file" id="title" placeholder="Enter title here" /> */}
						<div className="rounded-md w-full cursor-pointer h-[250px] flex flex-col justify-center items-center gap-3"
						style={{
							backgroundImage:`url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='7' ry='7' stroke='%2366666669' stroke-width='3' stroke-dasharray='10 24' stroke-dashoffset='0' stroke-linecap='round'/%3e%3c/svg%3e")`,
						}}
						>
							<Icons.UploadClould />
							<p className="text-sm text-gray-400">Drag and drop or Click to upload</p>
						</div>
					</div>
				</form>
				<div className="flex gap-3">
					<Button
						variant="outline"
						className="rounded-md text-[#376789] border-[#376789]"
						asChild
					>
						<Link href={`/home/organisations/${orgId}/projects/${projectId}/`}>
							 Cancel
						</Link>
					</Button>
					<Button className="rounded-md bg-[#376789] text-white">
						Add Dataset
					</Button>
				</div>
			</div>
		</main>
	);
}

;

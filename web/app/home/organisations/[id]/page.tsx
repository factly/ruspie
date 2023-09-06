import React from "react";
import { data } from "@/lib/data";
import Icons from "../../../../components/icons";
import { Button } from "../../../../components/ui/Button";
import Projects from "../../../../components/ui/Projects";
import Avatar from "../../../../components/ui/Avatar";

async function getData(id: string) {
	// get org data from api

	return data.find((org) => org.id === id);
}

export default async function Page({ params }: { params: { id: string } }) {
	const org = await getData(params.id);

	const handleFilterProject = (query: string) => {
		console.log(query);
	};

	const handleProjectClick = () => {
		console.log("project clicked");
	};

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<div className="flex flex-row gap-3">
					<Avatar src={org?.logo || ""} alt={`logo of ${org?.title}`} />
					<h1 className="text-xl font-semibold"> {org?.title} </h1>
				</div>
				<div className="flex flex-col w-2/5 justify-around gap-10">
					<Projects org={org || null} />
				</div>
				<Button className="rounded-md bg-[#376789] text-white">
					<Icons.PlusIcon /> Add Organization
				</Button>
			</div>
		</main>
	);
}

export async function generateStaticParams() {
	// const posts = await fetch('https://.../posts').then((res) => res.json())
	return data.map((org) => ({
		params: {
			id: org.id,
		},
	}));
}

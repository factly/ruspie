import React from "react";
import { data } from "@/lib/data";
import Icons from "../../../../components/icons";
import { Button } from "../../../../components/ui/Button";
import Projects from "../../../../components/ui/Projects";
import Avatar from "../../../../components/ui/Avatar";
import Link from "next/link";

async function getData(id: string) {
	// get org data from api
	console.log(id);
	return data.find((org) => org.id === id);
}

export default async function Page({ params }: { params: { organisationid: string } }) {
	const org = await getData(params.organisationid);

	const handleFilterProject = (query: string) => {
		console.log(query);
	};

	const handleProjectClick = () => {
		console.log("project clicked");
	};
	// console.log(org);

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<div className="flex flex-row gap-3">
					{/* <Avatar src={org?.logo || ""} alt={`logo of ${org?.title}`} /> */}
					<h1 className="text-xl font-semibold"> {org?.title} </h1>
				</div>
				<div className="flex flex-col w-2/5 justify-around gap-10">
					<Projects org={org || null} />
				</div>
				<Button className="rounded-md bg-[#376789] text-white" asChild>
					<Link href={`/home/organisations/${org?.id}/projects/new`}>
						<Icons.PlusIcon /> Add Projects
					</Link>
				</Button>
			</div>
		</main>
	);
}

export async function generateStaticParams() {
	// const posts = await fetch('https://.../posts').then((res) => res.json())
	return data.map((org) => ({
		params: {
			organisationid: org.id,
		},
	}));
}

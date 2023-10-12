"use client";
import React, { useEffect } from "react";
import { SearchBar } from "@/components/ui/searchBar";
import { Organisation } from "../../../components/ui/Organisation";
import { Organisation as OrganisationType } from "@/types/organisation";
import { Button } from "../../../components/ui/Button";
import Icons from "../../../components/icons";
import Link from "next/link";
import axios, { AxiosResponse } from "axios";
import { toast } from "react-hot-toast";
import { Loader } from "lucide-react";
import { useOrganisationsStore } from "@/lib/zustand/organisation";

export default function Page() {
	const [loading, setLoading] = React.useState(true);
	const { organisations, setOrganisations } = useOrganisationsStore();

	useEffect(() => {
		async function getOrganisations() {
			try {
				setLoading(true);
				const resp: AxiosResponse<{
					code: number;
					organisations: OrganisationType[];
				}> = await axios.get("/api/organisations");
				setOrganisations(resp.data.organisations);
			} catch (err) {
				toast.error("Error getting organisations");
			} finally {
				setLoading(false);
			}
		}
		getOrganisations();
	}, []);
	const [selectedOrg, setSelectedOrg] = React.useState<string | null>(null);

	const handleFilterOrg = async (query: string) => {
		try {
			setLoading(true);
			const resp: AxiosResponse<{
				code: number;
				organisations: OrganisationType[];
			}> = await axios.get(`/api/organisations?search_query=${query}`);
			setOrganisations(resp.data.organisations);
		} catch (err) {
			toast.error("Error getting organisations");
		} finally {
			setLoading(false);
		}
	};

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<h1 className="text-xl font-semibold"> Organizations </h1>
				<div className="flex flex-col w-2/5 justify-around gap-10">
					<SearchBar
						placeholder="Search Organisation"
						callback={handleFilterOrg}
					/>
					{loading ? (
						<div className="h-screen flex items-center justify-center -mt-28">
							<Loader className="h-10 w-10 animate-spin text-gray-400" />
						</div>
					) : (
						organisations.length !== 0 && (
							<>
								<div className="flex flex-col items-center gap-6 max-h-[60vh] overflow-y-auto">
									{organisations.map((org) => (
										<Organisation
											org={org}
											key={org.id}
											isOpen={selectedOrg === org.id}
											setIsOpen={() => {
												setSelectedOrg((prev) =>
													prev === org.id ? null : org.id,
												);
											}}
										/>
									))}
								</div>
							</>
						)
					)}
				</div>
				<Button className="rounded-md bg-[#376789] text-white" asChild>
					<Link href="/home/organisations/new">
						<Icons.PlusIcon /> Add Organization
					</Link>
				</Button>
			</div>
			{organisations.length === 0 && !loading && (
				<div className="flex flex-col items-center gap-4 my-auto w-full">
					<Icons.NotFound />
					<p className="text-xl w-fit font-medium">
						Oops! nothing found. Get started by creating new organization
					</p>
				</div>
			)}
		</main>
	);
}

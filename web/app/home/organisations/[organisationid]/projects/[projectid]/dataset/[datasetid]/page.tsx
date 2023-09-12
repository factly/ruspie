'use client';
import { Button } from "@/components/ui/Button";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { useState } from "react";
import { FeatureType } from "@/types/feature";
import Feature from "@/components/ui/feature";

export default function Page() {

	const features: FeatureType[] = [
		{ label: 'Search query', value: 'search' },
		{ label: 'REST API', value: 'rest' },
		{ label: 'GraphQL', value: 'graphql' },
		{ label: 'Schema', value: 'schema' },
	];

	const [selectedFeature, setSelectedFeature] = useState(features[0]);

	const handleFeatureChange = (value: FeatureType) => {
		setSelectedFeature(value);
	};

	return (
		<main className="flex flex-col mt-10 bg-transparent px-8">
			<div className="flex flex-row justify-between ml-auto md:w-[60%]">
				{/* select feature */}
				<Select
					onValueChange={(value) =>
						handleFeatureChange({
							label: "organisation",
							value,
						})
					}
				>
					<SelectTrigger className="w-[300px]">
						<SelectValue placeholder="Select a Feature" />
					</SelectTrigger>
					<SelectContent>
						{
							features.map(({ label, value }) => (
								<SelectItem value={value} key={value}>
									{label}
								</SelectItem>
							))
						}
					</SelectContent>
				</Select>
				{/* select Dataset */}
				<div className="flex flex-col items-end">
					<div>
						<Select>
							<SelectTrigger className="w-[200px]">
								<SelectValue placeholder="Select a Dataset" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="new">
									New Dataset
								</SelectItem>
							</SelectContent>
						</Select>
					</div>
					<Button variant='link' className="px-2 text-[#0022D4]">Try with your own data</Button>
				</div>
			</div>
			<Feature feature={selectedFeature} />
		</main>
	)
}

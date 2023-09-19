import Icons from '@/components/icons'
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@/components/ui/Avatar"
import { Button } from '@/components/ui/Button'
import Link from 'next/link'
import DatasetTable from '@/components/ui/DatasetTable'

export default function Page() {
	const org = {
		id: '1',
		logo: '',
		title: 'Org Title',
	}
	const project = {
		id: '1',
		title: 'Project Title',
	}
	const datasets = [
		{
			id: '1',
			title: 'Dataset Title 1',
			updatedAt: '2021-09-01',
			createdBy: 'User 1',
		},
		{
			id: '2',
			title: 'Dataset Title 2',
			updatedAt: '2021-09-02',
			createdBy: 'User 2',
		},
		{
			id: '3',
			title: 'Dataset Title 3',
			updatedAt: '2021-09-03',
			createdBy: 'User 3',
		},
		{
			id: '4',
			title: 'Dataset Title 4',
			updatedAt: '2021-09-04',
			createdBy: 'User 4',
		},
		{
			id: '5',
			title: 'Dataset Title 5',
			updatedAt: '2021-09-05',
			createdBy: 'User 5',
		},
		{
			id: '6',
			title: 'Dataset Title 6',
			updatedAt: '2021-09-06',
			createdBy: 'User 6',
		},
		{
			id: '7',
			title: 'Dataset Title 7',
			updatedAt: '2021-09-07',
			createdBy: 'User 7',
		},
		{
			id: '8',
			title: 'Dataset Title 8',
			updatedAt: '2021-09-08',
			createdBy: 'User 8',
		},
		{
			id: '9',
			title: 'Dataset Title 9',
			updatedAt: '2021-09-09',
			createdBy: 'User 9',
		},
		{
			id: '10',
			title: 'Dataset Title 10',
			updatedAt: '2021-09-10',
			createdBy: 'User 10',
		},
		{
			id: '11',
			title: 'Dataset Title 11',
			updatedAt: '2021-09-11',
			createdBy: 'User 11',
		},
		{
			id: '12',
			title: 'Dataset Title 12',
			updatedAt: '2021-09-12',
			createdBy: 'User 12',
		},
		{
			id: '13',
			title: 'Dataset Title 13',
			updatedAt: '2021-09-13',
			createdBy: 'User 13',
		},
		{
			id: '14',
			title: 'Dataset Title 14',
			updatedAt: '2021-09-14',
			createdBy: 'User 14',
		},
		{
			id: '15',
			title: 'Dataset Title 15',
			updatedAt: '2021-09-15',
			createdBy: 'User 15',
		},
		{
			id: '16',
			title: 'Dataset Title 16',
			updatedAt: '2021-09-16',
			createdBy: 'User 16',
		},
		{
			id: '17',
			title: 'Dataset Title 17',
			updatedAt: '2021-09-17',
			createdBy: 'User 17',
		},
		{
			id: '18',
			title: 'Dataset Title 18',
			updatedAt: '2021-09-18',
			createdBy: 'User 18',
		},
		{
			id: '19',
			title: 'Dataset Title 19',
			updatedAt: '2021-09-19',
			createdBy: 'User 19',
		},
		{
			id: '20',
			title: 'Dataset Title 20',
			updatedAt: '2021-09-20',
			createdBy: 'User 20',
		},
		{
			id: '21',
			title: 'Dataset Title 21',
			updatedAt: '2021-09-21',
			createdBy: 'User 21',
		},
	]


	return (
		<main className="flex flex-col mt-10 bg-transparent px-8">
			<div className="flex flex-row justify-between items-start">
				<div className="flex flex-row gap-3 items-center">
					<Avatar>
						<AvatarImage src={org.logo} alt={`logo of organisation`} />
						<AvatarFallback>
							<Icons.DefaultOrganisation />
						</AvatarFallback>
					</Avatar>
					<Link href={`/home/organisations/${org?.id}`}>
						<h1 className="text-xl font-semibold text-gray-600 hover:text-black cursor-pointer"> {org?.title} </h1>
					</Link>
					{">"}
					<h1 className="text-xl font-semibold"> Project Title </h1>
				</div>
				<Button className="rounded-md bg-[#376789] text-white gap-2" asChild>
					<Link href={`/home/organisations/${org?.id}/projects/${project.id}/dataset/new`}>
						<Icons.PlusIcon /> New Dataset
					</Link>
				</Button>
			</div>
			{
				datasets.length === 0 ? (
					<div className="flex flex-col items-center gap-4 my-auto w-full">
						<Icons.NotFound />
						<p className="text-xl w-fit font-medium">
							Oops! nothing found. Get started by creating new Dataset
						</p>
					</div>
				) : (
					<DatasetTable org={org} project={project} datasets={datasets} />
				)
			}
		</main>
	)
}



import Icons from '@/components/icons'
import Avatar from '@/components/ui/Avatar'
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
			title: 'Dataset Title',
			updatedAt: '2021-09-01',
			createdBy: 'User 1',
		},
		{
			id: '2',
			title: 'Dataset Title',
			updatedAt: '2021-09-01',
			createdBy: 'User 1',
		}
	]

	return (
		<main className="flex flex-col mt-10 bg-transparent px-8">
			<div className="flex flex-row justify-between items-start">
				<div className="flex flex-row gap-3 items-center">
					<Avatar src={org?.logo || ""} alt={`logo of ${org?.title}`} />
					<h1 className="text-xl font-semibold text-gray-600 hover:text-black cursor-pointer"> {org?.title} </h1>
					{">"}
					<h1 className="text-xl font-semibold"> Project Title </h1>
				</div>
				<Button className="rounded-md bg-[#376789] text-white gap-2" asChild>
					<Link href={`/home/organisations/${org?.id}/projects/${project.id}/dataset/new`}>
						<Icons.PlusIcon /> New Dataset
					</Link>
				</Button>
			</div>
			<DatasetTable org={org} project={project} datasets={datasets} />
		</main>
	)
}



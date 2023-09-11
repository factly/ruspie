import Icons from '@/components/icons'
import Avatar from '@/components/ui/Avatar'
import { Button } from '@/components/ui/Button'
import Link from 'next/link'
import DatasetTable from '@/components/ui/DatasetTable'
import defaultOrgLogo from '@/assets/defaultOrg.svg'

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
					<Avatar src={org?.logo || defaultOrgLogo} alt={`logo of ${org?.title}`} />
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



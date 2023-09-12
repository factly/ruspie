'use client';
import Icons from '@/components/icons'
import { Button } from '@/components/ui/Button'
import Link from 'next/link'
import React from 'react'
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table"
import DeleteButttonWithConfimModal from './DeleteButttonWithConfimModal';

// Todo: add props type
function DatasetTable({ org, project, datasets }: { org: any, project: any, datasets: any[] }) {

	const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
    event.stopPropagation();
  };


	return (
		<Table className='mx-auto mt-10 w-9/10 md:w-4/5'>
			<TableHeader>
				<TableRow className='bg-transparent text-[#666666] border-t rounded-t-md border-x'>
					<TableHead className='p-4'>Dataset Name</TableHead>
					<TableHead className='p-4'>Updated at</TableHead>
					<TableHead className='p-4'>Created by</TableHead>
					<TableHead className='p-4 text-center'>Actions</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody className='bg-white'>
				{
					datasets.map((dataset) => (
						<TableRow key={dataset.id}>
							<TableCell className='p-4'>{dataset.title}</TableCell>
							<TableCell className='p-4 text-[#1e1e1e]'>{dataset.updatedAt}</TableCell>
							<TableCell className='p-4 text-[#1e1e1e]'>{dataset.createdBy}</TableCell>
							<TableCell className='p-4'>
								<div className='flex flex-row gap-2 justify-center'>
									<Button
										variant="outline"
										size="icon"
										className="rounded border border-[#E6E6E6]"
										asChild
									>
										<Link href={`/home/organisations/${org?.id}/projects/${project.id}/dataset/1`}>
											<Icons.ArrowRight />
										</Link>
									</Button>
									<DeleteButttonWithConfimModal
										onButtonClick={handleDeleteClick}
										onConfirm={() => { }}
										onCancel={() => { }}
									/>
								</div>
							</TableCell>
						</TableRow>
					))
				}
			</TableBody>
		</Table>
	)
}

export default DatasetTable

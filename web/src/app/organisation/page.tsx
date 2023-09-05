// 'use client';
import React, { useEffect } from 'react'
import { SearchBar } from '@/components/ui/SearchBar'
import { Organisation } from '@/components/ui/Organisation';
import { Button } from '@/components/ui/button';
import Icons from '@/components/icons';

function Page() {
	const [data, setData] = React.useState([
		{
			id: '1',
			title: 'Organization 1',
			logo: '',
			createdAt: "May 1, 2021",
			updatedAt: "May 1, 2021",
			projects: [{
				id: '1',
				title: 'Project 1',
				createdAt: "May 1, 2021",
				updatedAt: "May 1, 2021",
			},
			{
				id: '2',
				title: 'Project 2',
				createdAt: "May 1, 2021",
				updatedAt: "May 1, 2021",
			},
			{
				id: '3',
				title: 'Project 3',
				createdAt: "May 1, 2021",
				updatedAt: "May 1, 2021",
			},],
		}
	])

	// useEffect(() => {
	// 	fetch('/api/organisation')
	// 		.then((res) => {
	// 			if (res.ok) {
	// 				return res.json()
	// 			}
	// 			throw new Error('Network response was not ok.')
	// 			// TODO: handle error toasts
	// 		})
	// 		.then((data) => setData(data))
	// }, [])
	console.log(data)
	return (
		<main className='flex flex-col mt-10'>
			<div className='flex flex-row justify-around items-start'>
				<h1 className='text-xl font-semibold'> Organization </h1>
				<div className='flex flex-col w-2/5 justify-around gap-10'>
					<SearchBar
						placeholder='Search Organisation'
						callback={() => {
							console.log("heehehe")
						}}
					/>
					<div className='flex flex-col items-center gap-6'>
						{data.map((org) => (
							<Organisation org={org} key={org.id} />
						))}
					</div>
				</div>
				<Button className='rounded-md bg-[#376789] text-white'>
					<Icons.PlusIcon /> Add Organization
				</Button>
			</div>
		</main>
	)
}

export default Page


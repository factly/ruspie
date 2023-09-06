'use client';
import React, { useEffect } from 'react'
import { SearchBar } from '@/components/ui/SearchBar'
import { Organisation } from '../../components/ui/Organisation';
import { Button } from '../../components/ui/Button';
import Icons from '../../components/icons';
import { data } from '@/lib/data';

function Page() {
	const [organisations, setOrganisations] = React.useState(data)

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
	const [selectedOrg, setSelectedOrg] = React.useState<string | null>(null)

	const handleFilterOrg = (query: string) => {
		//
		console.log(query)
	}


	return (
		<main className='flex flex-col mt-10 bg-transparent'>
			<div className='flex flex-row justify-around items-start'>
				<h1 className='text-xl font-semibold'> Organizations </h1>
				<div className='flex flex-col w-2/5 justify-around gap-10'>
					<SearchBar
						placeholder='Search Organisation'
						callback={handleFilterOrg}
					/>
					<div className='flex flex-col items-center gap-6'>
						{organisations.map((org) => (
							<Organisation
								org={org}
								key={org.id}
								isOpen={selectedOrg === org.id}
								setIsOpen={() => { setSelectedOrg(org.id) }}
							/>
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


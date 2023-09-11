'use client'
import React from 'react'
import { SearchBar } from '../searchBar'
import { Button } from '../Button'
import { Switch } from '../switch'
import Icons from '@/components/icons'

function SearchSql() {
	const [generatedSql, setGeneratedSql] = React.useState(false)

	const showGeneratedSql = () => {
		setGeneratedSql(!generatedSql)
	}

	return (
		<div className='flex flex-col gap-6'>
			<div className='mt-2 flex justify-between w-full gap-6'>
				<SearchBar
					inputClassName='!text-left py-3'
					wrapperClassName='w-full'
					withoutPrefixIcon={true} placeholder='Search SQL'
					callback={(value) => console.log(value)}
				/>
				<Button className="rounded-md bg-[#376789] text-white px-6 py-3">
					Query
				</Button>
			</div>
			<div className='flex justify-between'>
				<div className='flex gap-6'>
					<p>Search Results in:</p>
					<div className='flex gap-2'>
						Schema
						<Switch />
						Rows
					</div>
				</div>
				<Button variant='link' className="px-2 flex gap-3" onClick={showGeneratedSql}>
					Generated with SQL
					{generatedSql ? <Icons.ChevronDownIcon /> : <Icons.ChevronRightIcon size="large" />}
				</Button>
			</div>
			{
				generatedSql &&
				<div className='px-10 py-6 bg-gray-100'>
					SELECT * FROM <span className='text-[#376789]'>table_name</span> WHERE <span className='text-[#376789]'>column_name</span> = <span className='text-[#376789]'>value</span>
				</div>
			}
		</div>
	)
}

export default SearchSql

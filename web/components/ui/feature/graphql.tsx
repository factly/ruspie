import { Textarea } from '@/components/dataEntry/textarea'
import React from 'react'

function Graphql() {
	return (
		<div className='w-full flex flex-row gap-6 h-full'>
			<div className='w-full bg-[#fff] px-5 py-8'>
				<div className='flex flex-row justify-between items-center'>
					<h2 className='text-lg font-semibold'> Query </h2>
					<button className='rounded-md bg-[#376789] text-white px-4 py-2'> Query </button>
				</div>
				<Textarea className='w-full bg-[#fff] min-h-[65vh] border-none mt-3 p-0' placeholder='Enter Query here...'/>
			</div>
			<Textarea className='w-full bg-[#fff] min-h-[75vh] px-5 py-8' />
		</div>
	)
}

export default Graphql

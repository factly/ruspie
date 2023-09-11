import { Textarea } from '@/components/dataEntry/textarea'
import React from 'react'
import { Button } from '../Button'
import Icons from '@/components/icons'

function Graphql() {
	const [showSchema, setShowSchema] = React.useState(false)



	return (
		<div className='w-full flex flex-row gap-6 h-full'>
			<div className='w-full bg-[#fff] px-5 py-8 transition-all duration-300 ease-linear'>
				<div className='flex flex-row justify-between items-center'>
					<h2 className='text-lg font-semibold'> Query </h2>
					<Button className='rounded-md bg-[#376789] text-white px-4 py-2'> Execute </Button>
				</div>
				<Textarea className='w-full bg-[#fff] min-h-[65vh] border-none mt-3' placeholder='Enter Query here...' />
			</div>
			<div className='w-full bg-[#fff] px-5 py-8 transition-all duration-300 ease-linear'>
				<div className='flex flex-row justify-between items-center'>
					<h2 className='text-lg font-semibold'> Results </h2>
					<Button variant='outline' className='rounded-md border-[#376789] px-4 py-2'
						onClick={() => setShowSchema(!showSchema)}>
						{showSchema ? 'Hide Schema' : 'Show Schema'} <Icons.ChevronRightIcon />
					</Button>
				</div>
				<Textarea className='w-full bg-[#fff] min-h-[65vh] border-none mt-3' placeholder='Enter Query here...' readOnly/>
			</div>
			<div className={`bg-[#fff] transition-all duration-300 ease-linear ${showSchema ? 'w-4/5 px-5 py-8 ' : 'h-0 overflow-hidden w-0'}`}>
				{
					showSchema && (
						<>
							<div className='flex flex-row justify-between items-center'>
								<h2 className='text-lg font-semibold'> Schema </h2>
							</div>
							<Textarea className='w-full bg-[#fff] min-h-[65vh] border-none mt-3' placeholder='Enter Query here...' readOnly/>
						</>
					)
				}
			</div>
		</div>
	)
}

export default Graphql

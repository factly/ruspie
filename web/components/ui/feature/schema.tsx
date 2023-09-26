import { Textarea } from '@/components/dataEntry/textarea'
import React from 'react'
import { Button } from '../Button';
import Icons from '@/components/icons';
import { fetchSchemaForTable } from '@/lib/actions/features/getSchema';

export default function Schema() {
	const [schema, setSchema] = React.useState('');

	const dataset = {
		api_id: 'dummy'
	}
	const getSchema = () => {
		fetchSchemaForTable(dataset.api_id, true)
			.then((schema) => {
				setSchema(JSON.stringify(schema, null, 2));
				// setTableData(schema);
			})
			.catch((err) => console.log(err));
	}
	return (
		<div className='w-full flex flex-row justify-center gap-16 h-full'>
			<div className='w-2/4 flex flex-col justify-end items-end bg-white p-3'>
				<Textarea className='w-full min-h-[70vh] border-none mt-3' readOnly
					value={schema}
				/>
				<Button className='bg-[#376789] text-[#fff] mt-3' onClick={getSchema}>
					Get Schema
				</Button>
			</div>
		</div>
	)
}


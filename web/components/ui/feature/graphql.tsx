import { Textarea } from '@/components/dataEntry/textarea'
import React, { useEffect } from 'react'
import { Button } from '../Button'
import Icons from '@/components/icons'
import { graphqlEndpoint } from '@/lib/constants/apiEndpoints'
import { fetchGraphqlQuery } from '@/lib/actions/features/graphql'
import Loading from '../loading'
import { fetchSchemaForTable } from '@/lib/actions/features/getSchema'

function Graphql() {
	const [showSchema, setShowSchema] = React.useState(false)
	const [loading, setLoading] = React.useState(false);
	const [userQuery, setUserQuery] = React.useState('');
	const [tableData, setTableData] = React.useState('');
	const [schema, setSchema] = React.useState('');
	const setInitialQuery = (datasetApiId: String) => {
		setUserQuery(
			`{
             ${datasetApiId}(limit:10) {
          #enter the fields you want to query
               }
           }`,
		);
	};

	//! we need a dataset from props along with api_id
	const dataset = {
		api_id: 'dummy'
	}
	useEffect(() => {
		if (dataset.api_id) {
			fetchSchemaForTable(dataset.api_id, true)
				.then((schema) => {
					setSchema(JSON.stringify(schema, null, 2));
					// setTableData(schema);
					setInitialQuery(dataset.api_id);
				})
				.catch((err) => console.log(err));
		}
	}, [dataset.api_id]);


	const onExecute = async () => {
		try {
			setLoading(true);
			const graphqlResult = await fetchGraphqlQuery(userQuery);
			console.log(graphqlResult, 'graphqlResult');
			setTableData(graphqlResult);
			setLoading(false);
		} catch (error) {
			console.log(error, 'error');
			// setTableData([]);
			setLoading(false);
		}
	};


	return (
		<div className='w-full flex flex-row gap-6 h-full'>
			<div className='w-full bg-[#fff] px-5 py-8 transition-all duration-300 ease-linear'>
				<div className='flex flex-row justify-between items-center'>
					<h2 className='text-lg font-semibold'> Query </h2>
					<Button
						className='rounded-md bg-[#376789] text-white px-4 py-2'
						onClick={onExecute}
					>
						Execute
					</Button>
				</div>
				<Textarea
					className='w-full bg-[#fff] min-h-[65vh] border-none mt-3'
					placeholder='Enter Query here...'
					onChange={(e) => {
						if (loading) return;
						setUserQuery(e.target.value)
					}}
					value={userQuery}
				/>

			</div>
			{
				loading ?
					<div className='w-full bg-[#fff] px-5 py-8 transition-all duration-300 ease-linear flex justify-center items-center'>
						<Loading />
					</div>
					:
					<div className='w-full bg-[#fff] px-5 py-8 transition-all duration-300 ease-linear'>
						<div className='flex flex-row justify-between items-center'>
							<h2 className='text-lg font-semibold'> Results </h2>
							<Button variant='outline' className='rounded-md border-[#376789] px-4 py-2'
								onClick={() => setShowSchema(!showSchema)}>
								{showSchema ?
									<div className='flex gap-2 items-center'>
										{'Hide Schema'}
										<Icons.ArrowRight2 />
									</div>
									:
									<div className='flex gap-2 items-center'>
										{'Show Schema'}
										<Icons.ArrowLeft2 />
									</div>}
							</Button>
						</div>
						<Textarea
							className='w-full bg-[#fff] min-h-[65vh] border-none mt-3'
							value={JSON.stringify(tableData, null, 2)}
							placeholder='Enter Query here...'
							readOnly
						/>
					</div>
			}
			<div className={`bg-[#fff] transition-all duration-300 ease-linear ${showSchema ? 'w-4/5 px-5 py-8 ' : 'h-0 overflow-hidden w-0'}`}>
				{
					showSchema && (
						<>
							<div className='flex flex-row justify-between items-center'>
								<h2 className='text-lg font-semibold'> Schema </h2>
							</div>
							<Textarea
							className='w-full bg-[#fff] min-h-[65vh] border-none mt-3' placeholder='Enter Query here...' readOnly
								value={schema}
							/>
						</>
					)
				}
			</div>
		</div>
	)
}

export default Graphql

'use client'
import React from 'react'
import { SearchBar } from '../searchBar'
import { Button } from '../Button'
import { Switch } from '../switch'
import Icons from '@/components/icons'
import { fetchRowsForTable, fetchSchemaForTable } from '@/lib/actions/features/getSchema'
import { convertTextToSql } from '@/lib/actions/features/textToSql'
import Loading from '../loading'
import { fetchSqlForQuery } from '@/lib/actions/features/searchSql'
import DatasetTable from '../DatasetTable'
import SqlResultTable from './SqlResultTable'

function SearchSql() {
	const [generatedSql, setGeneratedSql] = React.useState('')
	const [generatedSqlVisible, setGeneratedSqlVisible] = React.useState(false)
	const [loading, setLoading] = React.useState(false);
	const [userQuery, setUserQuery] = React.useState('');
	const [selectedOption, setSelectedOption] = React.useState('Schema');
	const [tableData, setTableData] = React.useState([]);

	const showGeneratedSql = () => {
		setGeneratedSqlVisible(!generatedSqlVisible)
	}

	//! we need a dataset from props along with api_id
	const dataset = {
		api_id: 'dummy'
	}

	const onSearch = async () => {
		try {
			setLoading(true);
			const schema = selectedOption === 'Schema' ? await fetchSchemaForTable(dataset.api_id) : null;
			const rows = selectedOption === 'Rows' ? await fetchRowsForTable(dataset.api_id) : null;
			// const sql = !editSql ? await convertTextToSql(dataset.api_id, schema, query , rows) : {response: editSql}
			const sql = await convertTextToSql({
				table: dataset.api_id,
				schema: schema,
				query: userQuery,
				rows: rows
			})
			setGeneratedSql(sql.response)
			const sqlResult = await fetchSqlForQuery(sql.response);
			setTableData(sqlResult);
			// setEditSql(null);
			setLoading(false);
		} catch (error) {
			console.log(error, 'error');
			// setEditSql(null);
			// setTableData([]);
			setLoading(false);
		}
	};

	return (
		<div className='flex flex-col gap-6 mb-10'>
			<div className='mt-2 flex justify-between w-full gap-6'>
				<SearchBar
					inputClassName='!text-left py-3'
					wrapperClassName='w-full'
					withoutPrefixIcon={true} placeholder='Search SQL'
					callback={(value) => setUserQuery(value)}
				/>
				<Button className="rounded-md bg-[#376789] text-white px-6 py-3" onClick={onSearch}>
					Query
				</Button>
			</div>
			<div className='flex justify-between'>
				<div className='flex gap-6'>
					<p>Search Results in:</p>
					<div className='flex gap-2'>
						Schema
						<Switch checked={selectedOption === 'Rows'} onCheckedChange={() => setSelectedOption(selectedOption === 'Rows' ? 'Schema' : 'Rows')} />
						Rows
					</div>
				</div>
				<Button variant='link' className="px-2 flex gap-3" onClick={showGeneratedSql}>
					Generated with SQL
					{generatedSql ? <Icons.ChevronDownIcon /> : <Icons.ChevronRightIcon size="large" />}
				</Button>
			</div>
			{
				generatedSqlVisible &&
				<div className='px-10 py-6 bg-white'>
					{loading ? <div className='flex justify-center items-center'><Loading /><div>Generating SQL...</div></div> :
						generatedSql !== '' ? generatedSql : 'No SQL generated yet! Please search for a query.'}
				</div>
			}
			<SqlResultTable data={tableData} itemsPerPage={6} />
		</div>
	)
}

export default SearchSql

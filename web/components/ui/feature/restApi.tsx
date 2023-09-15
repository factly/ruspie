'use client';
import { useState } from 'react';
import { Textarea } from '@/components/dataEntry/textarea'
import { Label } from '@/components/dataEntry/label'
import { Input } from '@/components/dataEntry/input'
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuTrigger,
	DropdownMenuCheckboxItem
} from "@/components/ui/dropdown-menu"
import React from 'react'
import { Button } from '../Button';
import Icons from '@/components/icons';
import { fetchSchemaForTable } from '@/lib/actions/features/getSchema';
import { restEndpoint } from '@/lib/constants/apiEndpoints';

export default function RestApi() {

	//! we need a dataset from props along with api_id
	const dataset = {
		api_id: 'dummy'
	}

	const [loading, setLoading] = useState(false);
	const [schemaLoading, setSchemaLoading] = useState(false);
	const [formData, setFormData] = useState({
		fileFormat: '',
		pageSize: 1,
		limit: 1,
		columns_to_retrieve: [],
	});

	const [schema, setSchema] = useState(null)
	// responseData stores the search data coming from ruspie and displays it in the response textarea
	const [responseData, setResponseData] = useState('');
	// use to fetch schema for the dataset
	React.useEffect(() => {
		if (dataset.api_id) {
			setSchemaLoading(true);
			fetchSchemaForTable(dataset.api_id, true)
				.then((schema) => {
					setSchemaLoading(false);
					setSchema(schema);
				})
				.catch((err) => {
					console.log(err);
					setSchemaLoading(false);
				});
		}
	}, [dataset.api_id]);


	const handleChange = (e: any) => {
		setFormData({
			...formData,
			[e.target.name]: e.target.value,
		})
	}

	const handleSubmit = (e) => {
		setResponseData('');
		e.preventDefault();
		let queryParam: any = {};
		queryParam.limit = formData.limit;
		queryParam.page = formData.pageSize;
		if (formData.columns_to_retrieve?.length > 0) {
			queryParam.columns_to_retrieve = '';
			queryParam.columns_to_retrieve = formData.columns_to_retrieve[0];
			for (let i = 1; i < formData.columns_to_retrieve.length; i++) {
				queryParam.columns_to_retrieve += `,${formData.columns_to_retrieve[i]}`;
			}
		}

		const URL = `${restEndpoint}/${dataset.api_id}?` + new URLSearchParams(queryParam)
		setLoading(true);
		fetch(URL)
			.then((res) => {
				if (res.status !== 200) {
					return res.json().then((data) => {
						throw Error(data?.message);
					});
				} else {
					return res.json();
				}
			})
			.then((responseData) => {
				setResponseData(JSON.stringify(responseData, null, 2));
				setLoading(false);
			})
			.catch((err) => {
				alert(err.message);
				setLoading(false);
			});
	};


	return (
		<div className='w-full flex flex-row justify-end gap-16 h-full'>
			<div className='px-4 w-2/5'>
				<form className="flex flex-col items-center w-4/5 gap-6">
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="fileFormat" className="font-normal">
							File Format
						</Label>
						<Input
							name="fileFormat"
							type="text"
							id="fileFormat"
							placeholder="Enter file format here"
							onChange={handleChange}
							value={formData.fileFormat}
						/>
					</div>
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="pageSize" className="font-normal">
							Page Size
						</Label>
						<Input
							name="pageSize"
							type="number"
							min={1}
							defaultValue={1}
							id="pageSize"
							placeholder="Enter page size here"
							onChange={handleChange}
							value={formData.pageSize}
						/>
					</div>
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="limit" className="font-normal">
							Limit
						</Label>
						<Input
							name="limit"
							type="number"
							min={1}
							defaultValue={1}
							id="limit"
							placeholder="Enter page size here"
							onChange={handleChange}
							value={formData.limit}
						/>
					</div>
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="columns_to_retrieve" className="font-normal">
							Columns to retrieve
						</Label>
						<DropdownMenu>
							<DropdownMenuTrigger className="w-full bg-white px-3 py-2 rounded-md border border-gray-200 text-left flex justify-between items-center">
								Columns To retrieve
							 <Icons.ChevronDownIcon />
							</DropdownMenuTrigger>
							<DropdownMenuContent className='h-36 overflow-y-auto bg-white w-[22rem]'>
								{
									schemaLoading ? <div>Loading...</div> :
										schema !== null &&
										schema?.fields?.map((field) => {
											if (field.name === undefined) return null;
											return (
												<DropdownMenuCheckboxItem
													className="w-50"
													key={field.name}
													checked={formData.columns_to_retrieve?.includes(field.name)}
													onCheckedChange={(checked) => {
														if (checked) {
															setFormData({
																...formData,
																columns_to_retrieve: [...formData.columns_to_retrieve, field.name]
															})
														} else {
															setFormData({
																...formData,
																columns_to_retrieve: formData.columns_to_retrieve.filter((item) => item !== field.name)
															})
														}
													}}
												>
													{field.name}
												</DropdownMenuCheckboxItem>
											)
										})
								}
							</DropdownMenuContent>
						</DropdownMenu>
					</div>
					<div className="flex w-full items-center justify-between">
						<Button className='rounded-md bg-[#376789] text-white px-4 py-2'
							onClick={handleSubmit}
						> Execute </Button>
						<Button variant='outline' className='rounded-md border-[#376789] text-[#376789] px-4 py-2'>
							<Icons.PlusIcon color="#376789" /> Add Filter
						</Button>
					</div>
				</form>
			</div >
			<Textarea className='w-2/4 bg-[#fff] min-h-[70vh] border-none mt-3' readOnly value={responseData} />
		</div >
	)
}


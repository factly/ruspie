import { Textarea } from '@/components/dataEntry/textarea'
import { Label } from '@/components/dataEntry/label'
import { Input } from '@/components/dataEntry/input'
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import React from 'react'
import { Button } from '../Button';
import Icons from '@/components/icons';

export default function Schema() {
	return (
		<div className='w-full flex flex-row justify-end gap-16 h-full'>
			<div className='px-4 w-2/5'>
				<form className="flex flex-col items-center w-4/5 gap-6">
					<div className="grid w-full items-center gap-3">
						<Label htmlFor="feature" className="font-normal">
							Feature
						</Label>
						<Select
						// onValueChange={(value) =>
						// 	handleChange({
						// 		name: "organisation",
						// 		value,
						// 	})
						// }
						>
							<SelectTrigger className="w-full">
								<SelectValue placeholder="Select a Organisation" />
							</SelectTrigger>
							<SelectContent>
								<SelectGroup>
									<SelectLabel>Rest Api</SelectLabel>
									<SelectItem value="new">
										item 1
									</SelectItem>
									<SelectItem value="new2">
										item 2
									</SelectItem>
								</SelectGroup>
							</SelectContent>
						</Select>
						<div className="grid w-full items-center gap-3">
						<Label htmlFor="fileName" className="font-normal">
							File Name
						</Label>
						<Input
							name="fileName"
							type="text"
							id="fileName"
							placeholder="Enter file name here"
						// onChange={handleChange}
						// value={formData.title}
						/>
					</div>
					</div>
					<div className="flex w-full items-center justify-between">
						<Button className='rounded-md bg-[#376789] text-white px-4 py-2'> Execute </Button>
						<Button variant='outline' className='rounded-md border-[#376789] text-[#376789] px-4 py-2'>
							<Icons.PlusIcon color="#376789" /> Add Filter
						</Button>
					</div>
				</form>
			</div>
			<Textarea className='w-2/4 bg-[#fff] min-h-[70vh] border-none mt-3' readOnly />
		</div>
	)
}


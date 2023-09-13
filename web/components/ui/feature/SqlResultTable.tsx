import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table"
import { NotFound } from '@/components/icons/notFound';

import React, { useState } from 'react';

type Props = {
  data: any[];
  itemsPerPage: number;
};

function SqlResultTable({ data, itemsPerPage }: Props) {
  const [currentPage, setCurrentPage] = useState(1);

  if (data === null || data.length === 0) {
    return <NotFound />;
  }

  const tableHeaders = Object.keys(data[0]);

  // Calculate the total number of pages
  const totalPages = Math.ceil(data.length / itemsPerPage);

  // Calculate the start and end indices for the current page
  const startIndex = (currentPage - 1) * itemsPerPage;
  const endIndex = startIndex + itemsPerPage;

  // Slice the data to display only the current page's data
  const currentPageData = data.slice(startIndex, endIndex);

  const handlePageChange = (page: number) => {
    setCurrentPage(page);
  };

  return (
    <div>
      <Table className='mx-auto mt-10 w-9/10 md:w-3/5 overflow-x-auto'>
        <TableHeader>
          <TableRow className='bg-transparent text-[#666666] border-t rounded-t-md border-x'>
            {tableHeaders.map((header: string) => (
              <TableHead key={header} className='p-4 md:min-w-[150px]'>
                {header}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody className='bg-white'>
          {currentPageData.map((row: any) => (
            <TableRow key={row.id}>
              {tableHeaders.map((header: string) => (
                <TableCell key={header} className='p-4 md:min-w-[150px]'>
                  {row[header]}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {/* Pagination controls */}
      <div className='flex justify-center mt-4'>
        {Array.from({ length: totalPages }, (_, index) => (
          <button
            key={index}
            onClick={() => handlePageChange(index + 1)}
            className={`mx-2 px-3 py-1 rounded ${
              currentPage === index + 1 ? 'bg-blue-500 text-white' : 'bg-gray-300'
            }`}
          >
            {index + 1}
          </button>
        ))}
      </div>
    </div>
  );
}

export default SqlResultTable;


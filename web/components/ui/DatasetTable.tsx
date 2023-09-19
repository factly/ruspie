"use client";
import Icons from "@/components/icons";
import { Button } from "@/components/ui/Button";
import Link from "next/link";
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import DeleteButttonWithConfirmModal from "./DeleteButttonWithConfimModal";
import React, { useState } from 'react';
import { NotFound } from "../icons/notFound";

type Props = {
  org: any;
  project: any;
  datasets: any[];
};

function DatasetTable({
  org,
  project,
  datasets,
}: Props) {
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 5; // You can adjust the number of items per page

  if (datasets === null || datasets.length === 0) {
    return <NotFound />;
  }

  const tableHeaders = [
    'Dataset Name',
    'Updated at',
    'Created by',
    'Actions',
  ];

  // Calculate the total number of pages
  const totalPages = Math.ceil(datasets.length / itemsPerPage);

  // Calculate the start and end indices for the current page
  const startIndex = (currentPage - 1) * itemsPerPage;
  const endIndex = startIndex + itemsPerPage;

  // Slice the datasets to display only the current page's data
  const currentPageData = datasets.slice(startIndex, endIndex);

  const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
    event.stopPropagation();
  };

  const handlePageChange = (page: number) => {
    setCurrentPage(page);
  };

  return (
    <div>
      <Table className='mx-auto mt-10 w-9/10 md:w-4/5'>
        <TableHeader>
          <TableRow className='bg-transparent text-[#666666] border-t rounded-t-md border-x'>
            {tableHeaders.map((header: string) => (
              <TableHead key={header} className='p-4'>
                {header}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody className='bg-white'>
          {currentPageData.map((dataset) => (
            <TableRow key={dataset.id}>
              <TableCell className='p-4'>{dataset.title}</TableCell>
              <TableCell className='p-4 text-[#1e1e1e]'>
                {dataset.updatedAt}
              </TableCell>
              <TableCell className='p-4 text-[#1e1e1e]'>
                {dataset.createdBy}
              </TableCell>
              <TableCell className='p-4'>
                <div className='flex flex-row gap-2 justify-center'>
                  <Button
                    variant='outline'
                    size='icon'
                    className='rounded border border-[#E6E6E6]'
                    asChild
                  >
                    <Link
                      href={`/home/organisations/${org?.id}/projects/${project.id}/dataset/1`}
                    >
                      <Icons.ArrowRight />
                    </Link>
                  </Button>
                  <DeleteButttonWithConfirmModal
                    onButtonClick={handleDeleteClick}
                    onConfirm={async () => {}}
                    onCancel={() => {}}
                  />
                </div>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {/* Pagination controls */}
      <div className='flex justify-end mt-4 w-9/10 md:w-4/5 mx-auto'>
        <Button
          onClick={() => handlePageChange(currentPage - 1)}
					disabled={currentPage === 1}
          className={`px-3 py-1 rounded text-[#376789] ${
            currentPage === 1 ? 'text-gray-400 cursor-not-allowed' : ''
          } rotate-180`}
        >
          <Icons.ChevronRightIcon size="large"/>
        </Button>
        {Array.from({ length: totalPages }, (_, index) => (
          <Button
            key={index}
            onClick={() => handlePageChange(index + 1)}
            className={`mx-2 px-3 py-1 rounded text-[#376789] ${
              currentPage === index + 1 ? 'border border-[#376789]' : ''
            }`}
          >
            {index + 1}
          </Button>
        ))}
        <Button
          onClick={() => handlePageChange(currentPage + 1)}
					disabled={currentPage === totalPages}
          className={`px-3 py-1 rounded text-[#376789] ${
            currentPage === totalPages ? 'text-gray-400 cursor-not-allowed' : ''
          }`}
        >
          <Icons.ChevronRightIcon size="large"/>
        </Button>
      </div>
    </div>
  );
}

export default DatasetTable;

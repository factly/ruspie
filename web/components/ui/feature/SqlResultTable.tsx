import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { NotFound } from "@/components/icons/notFound";
import React, { useState } from "react";
import { Button } from "../Button";
import Icons from "@/components/icons";

type Props = {
  data: any[];
  itemsPerPage: number;
};

function SqlResultTable({ data, itemsPerPage }: Props) {
  const [currentPage, setCurrentPage] = useState(1);

  if (data === null || data.length === 0) {
    return (
      <div className="flex items-start justify-center">
        <NotFound />
      </div>
    );
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

  // Determine the range of page buttons to display
  const pageButtonRange = 5;
  const startPage = Math.max(currentPage - Math.floor(pageButtonRange / 2), 1);
  const endPage = Math.min(startPage + pageButtonRange - 1, totalPages);

  return (
    <div>
      <Table className="mx-auto mt-10 w-9/10 md:w-3/5 overflow-x-auto">
        <TableHeader>
          <TableRow className="bg-transparent text-[#666666] border-t rounded-t-md border-x">
            {tableHeaders.map((header: string) => (
              <TableHead key={header} className="p-4 md:min-w-[150px]">
                {header}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody className="bg-white">
          {currentPageData.map((row: any) => (
            <TableRow key={row.id}>
              {tableHeaders.map((header: string) => (
                <TableCell key={header} className="p-4 md:min-w-[150px]">
                  {row[header]}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {/* Pagination controls */}
      <div className="flex justify-end gap-2 mt-4">
        <Button
          onClick={() => handlePageChange(currentPage - 1)}
          disabled={currentPage === 1}
          className={`px-3 py-1 rounded text-[#376789] ${
            currentPage === 1 ? "text-gray-400 cursor-not-allowed" : ""
          } rotate-180`}
        >
          <Icons.ChevronRightIcon size="large" />
        </Button>
        <div className="flex gap-2">
          {Array.from({ length: endPage - startPage + 1 }, (_, index) => (
            <Button
              key={startPage + index}
              onClick={() => handlePageChange(startPage + index)}
              className={`px-3 py-1 rounded text-[#376789] ${
                currentPage === startPage + index
                  ? "border border-[#376789]"
                  : ""
              }`}
            >
              {startPage + index}
            </Button>
          ))}
        </div>
        <Button
          onClick={() => handlePageChange(currentPage + 1)}
          disabled={currentPage === totalPages}
          className={`px-3 py-1 rounded text-[#376789] ${
            currentPage === totalPages ? "text-gray-400 cursor-not-allowed" : ""
          }`}
        >
          <Icons.ChevronRightIcon size="large" />
        </Button>
      </div>
    </div>
  );
}

export default SqlResultTable;

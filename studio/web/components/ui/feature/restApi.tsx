"use client";
import { useState } from "react";
import { Textarea } from "@/components/dataEntry/textarea";
import { Label } from "@/components/dataEntry/label";
import { Input } from "@/components/dataEntry/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import React from "react";
import { Button } from "../Button";
import Icons from "@/components/icons";
import { fetchSchemaForTable } from "@/lib/actions/features/getSchema";
import { restEndpoint } from "@/lib/constants/apiEndpoints";
import { createFilter } from "@/lib/actions/features/restApi";
import { File } from "@/types/file";
import axios from "axios";
import { getNameFromUrl } from "@/lib/actions/features/getNameFromUrl";

export default function RestApi({ dataset }: { dataset: File }) {
  const [loading, setLoading] = useState(false);
  const [schemaLoading, setSchemaLoading] = useState(false);
  const [formData, setFormData] = useState({
    fileFormat: "",
    pageSize: 1,
    limit: 1,
    columns_to_retrieve: [],
  });

  // filterInputsFields stores the filter inputs
  type filterType = {
    columnName: string;
    operator: string;
    value: string;
  }[];

  const operatorList = [
    {
      label: ">",
      value: "gt=",
    },
    {
      label: "<",
      value: "lt=",
    },
    {
      label: "=",
      value: "=",
    },
    {
      label: ">=",
      value: "gte=",
    },
    {
      label: "<=",
      value: "lte=",
    },
  ];
  const [filterInputFields, setFilterInputFields] = useState<filterType>([
    {
      columnName: "",
      operator: "",
      value: "",
    },
  ]);

  const [schema, setSchema] = useState<any>(null);
  // responseData stores the search data coming from ruspie and displays it in the response textarea
  const [responseData, setResponseData] = useState("");
  // use to fetch schema for the dataset
  React.useEffect(() => {
    if (dataset.id) {
      setSchemaLoading(true);
      fetchSchemaForTable(dataset, true)
        .then((schema) => {
          setSchemaLoading(false);
          setSchema(schema);
        })
        .catch((err) => {
          console.log(err);
          setSchemaLoading(false);
        });
    }
  }, [dataset]);

  const handleChange = (e: any) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const addFilterFields = (e: any) => {
    e.preventDefault();
    setFilterInputFields([
      ...filterInputFields,
      {
        columnName: "",
        operator: "",
        value: "",
      },
    ]);
  };
  const handleFilterFormInputChange = (
    index: number,
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setFilterInputFields(
      filterInputFields.map((inputField, i) => {
        if (index === i) {
          return {
            ...inputField,
            [event.target.name]: event.target.value,
          };
        }
        return inputField;
      }),
    );
  };
  type SelectEvent = {
    name: string;
    value: string;
  };

  const handleFilterFormColumnChange = (index: number, event: SelectEvent) => {
    setFilterInputFields(
      filterInputFields.map((inputField, i) => {
        if (index === i) {
          return {
            ...inputField,
            columnName: event.value,
          };
        }
        return inputField;
      }),
    );
  };

  const handleFilterFormOperatorChange = (
    index: number,
    event: SelectEvent,
  ) => {
    setFilterInputFields(
      filterInputFields.map((inputField, i) => {
        if (index === i) {
          return {
            ...inputField,
            operator: event.value,
          };
        }
        return inputField;
      }),
    );
  };

  const removeFilterFields = (index: number) => {
    const values = [...filterInputFields];
    values.splice(index, 1);
    setFilterInputFields(values);
  };

  const handleSubmit = (e: any) => {
    setResponseData("");
    e.preventDefault();
    let queryParam: any = {};
    queryParam.limit = formData.limit;
    queryParam.page = formData.pageSize;
    if (formData.columns_to_retrieve?.length > 0) {
      queryParam.columns_to_retrieve = "";
      queryParam.columns_to_retrieve = formData.columns_to_retrieve[0];
      for (let i = 1; i < formData.columns_to_retrieve.length; i++) {
        queryParam.columns_to_retrieve += `,${formData.columns_to_retrieve[i]}`;
      }
    }
    const name = getNameFromUrl(dataset.s3_url);
    const URL =
      `${restEndpoint}/${name}?` +
      new URLSearchParams(queryParam) +
      createFilter(filterInputFields, schema);
    setLoading(true);
    // fetch(URL, { headers: { "FILE-EXT": dataset.extenstion } })
    //   .then(async (res) => {
    //     if (res.status !== 200) {
    //       const data = await res.json();
    //       throw Error(data?.message);
    //     } else {
    //       return res.json();
    //     }
    //   })
    //   .then((responseData) => {
    //     setResponseData(JSON.stringify(responseData, null, 2));
    //     setLoading(false);
    //   })
    //   .catch((err) => {
    //     alert(err.message);
    //     setLoading(false);
    //   });
    console.log(dataset.extension, dataset.s3_url);
    axios
      .get(URL, {
        headers: { "FILE-EXT": dataset.extension },
      })
      .then((res) => {
        if (res.status !== 200) {
          const data = res.data;
          throw Error(data?.message);
        } else {
          return res.data;
        }
      })
      .then((data) => {
        setResponseData(JSON.stringify(data, null, 4));
        setLoading(false);
      })
      .catch((err) => {
        alert(err.message);
        setLoading(false);
      });
  };

  return (
    <div className="w-full flex flex-row justify-end gap-16 h-full max-h-[80vh] overflow-x-auto mb-10">
      <div className="px-4 w-3/6">
        <form className="flex flex-col items-center w-4/5 gap-6">
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="pageSize" className="font-normal">
              Page
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
          {/* <div className="grid w-full items-center gap-3"> */}
          {/*   <Label htmlFor="columns_to_retrieve" className="font-normal"> */}
          {/*     Columns to retrieve */}
          {/*   </Label> */}
          {/*   <DropdownMenu> */}
          {/*     <DropdownMenuTrigger className="w-full bg-white px-3 py-2 rounded-md border border-gray-200 text-left flex justify-between items-center"> */}
          {/*       Columns To retrieve */}
          {/*       <Icons.ChevronDownIcon /> */}
          {/*     </DropdownMenuTrigger> */}
          {/*     <DropdownMenuContent className="h-36 overflow-y-auto bg-white w-[22rem]"> */}
          {/*       {schemaLoading ? ( */}
          {/*         <div>Loading...</div> */}
          {/*       ) : ( */}
          {/*         schema !== null && */}
          {/*         schema?.fields?.map((field) => { */}
          {/*           if (field.name === undefined) return null; */}
          {/*           return ( */}
          {/*             <DropdownMenuCheckboxItem */}
          {/*               className="w-50" */}
          {/*               key={field.name} */}
          {/*               checked={formData.columns_to_retrieve?.includes( */}
          {/*                 field.name, */}
          {/*               )} */}
          {/*               onCheckedChange={(checked) => { */}
          {/*                 if (checked) { */}
          {/*                   setFormData({ */}
          {/*                     ...formData, */}
          {/*                     columns_to_retrieve: [ */}
          {/*                       ...formData.columns_to_retrieve, */}
          {/*                       field.name, */}
          {/*                     ], */}
          {/*                   }); */}
          {/*                 } else { */}
          {/*                   setFormData({ */}
          {/*                     ...formData, */}
          {/*                     columns_to_retrieve: */}
          {/*                       formData.columns_to_retrieve.filter( */}
          {/*                         (item) => item !== field.name, */}
          {/*                       ), */}
          {/*                   }); */}
          {/*                 } */}
          {/*               }} */}
          {/*             > */}
          {/*               {field.name} */}
          {/*             </DropdownMenuCheckboxItem> */}
          {/*           ); */}
          {/*         }) */}
          {/*       )} */}
          {/*     </DropdownMenuContent> */}
          {/*   </DropdownMenu> */}
          {/* </div> */}
          <div className="grid w-full items-center gap-3">
            {filterInputFields.length > 0 && (
              <Label htmlFor="filter" className="font-normal">
                Filters
              </Label>
            )}
            {filterInputFields.map((inputField, index) => (
              <div
                className="flex w-full items-center justify-between gap-3"
                key={index}
              >
                <Select
                  onValueChange={(value) => {
                    handleFilterFormColumnChange(index, {
                      name: "columnName",
                      value,
                    });
                  }}
                >
                  <SelectTrigger className="w-[50%]">
                    <SelectValue placeholder="Column" />
                  </SelectTrigger>
                  <SelectContent className="h-44 overflow-y-auto bg-white w-[180px]">
                    {schemaLoading ? (
                      <div>Loading...</div>
                    ) : (
                      schema !== null &&
                      schema?.fields?.map((field: any) => {
                        if (field.name === undefined) return null;
                        return (
                          <SelectItem value={field.name} key={field.name}>
                            {field.name}
                          </SelectItem>
                        );
                      })
                    )}
                  </SelectContent>
                </Select>
                <Select
                  onValueChange={(value) => {
                    handleFilterFormOperatorChange(index, {
                      name: "operator",
                      value,
                    });
                  }}
                >
                  <SelectTrigger className="w-[20%]">
                    <SelectValue placeholder="Operator" />
                  </SelectTrigger>
                  <SelectContent className="h-44 overflow-y-auto bg-white">
                    {operatorList.map((operator) => (
                      <SelectItem value={operator.value} key={operator.value}>
                        {operator.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input
                  name="value"
                  className="w-[50%]"
                  type="text"
                  id="value"
                  value={inputField.value}
                  placeholder="value"
                  onChange={(event) =>
                    handleFilterFormInputChange(index, event)
                  }
                />
                <Button
                  type="button"
                  variant="ghost"
                  className="cursor-pointer p-2"
                  onClick={() => removeFilterFields(index)}
                >
                  <Icons.ChevronDownIcon />
                </Button>
              </div>
            ))}
          </div>
          <div className="flex w-full items-center justify-between">
            <Button
              className="rounded-md bg-[#376789] text-white px-4 py-2"
              onClick={handleSubmit}
            >
              Execute
            </Button>
            <Button
              variant="outline"
              type="button"
              className="rounded-md border-[#376789] text-[#376789] px-4 py-2"
              onClick={addFilterFields}
            >
              <Icons.PlusIcon color="#376789" /> Add Filter
            </Button>
          </div>
        </form>
      </div>
      <Textarea
        className="w-2/4 bg-[#fff] min-h-[70vh] border-none mt-3"
        readOnly
        value={responseData}
      />
    </div>
  );
}

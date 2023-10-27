"use client";
import React from "react";
import { SearchBar } from "../searchBar";
import { Button } from "../Button";
import { Switch } from "../switch";
import Icons from "@/components/icons";
import {
  fetchRowsForTable,
  fetchSchemaForTable,
} from "@/lib/actions/features/getSchema";
import { convertTextToSql } from "@/lib/actions/features/textToSql";
import Loading from "../loading";
import { fetchSqlForQuery } from "@/lib/actions/features/searchSql";
import SqlResultTable from "./SqlResultTable";
import { File } from "@/types/file";
import { getNameFromUrl } from "@/lib/actions/features/getNameFromUrl";
import { toast } from "react-hot-toast";
import { textToSqlEnabled } from "@/lib/utils/textToSqlEnabled";

function SearchSql({ dataset }: { dataset: File }) {
  const [generatedSql, setGeneratedSql] = React.useState("");
  const [generatedSqlVisible, setGeneratedSqlVisible] = React.useState(false);
  const [loading, setLoading] = React.useState(false);
  const [userQuery, setUserQuery] = React.useState("");
  const [selectedOption, setSelectedOption] = React.useState("Schema");
  const [tableData, setTableData] = React.useState([]);

  const showGeneratedSql = () => {
    setGeneratedSqlVisible(!generatedSqlVisible);
  };

  //! we need a dataset from props along with api_id
  const onSearch = async () => {
    try {
      setLoading(true);
      if (textToSqlEnabled) {
        const schema =
          selectedOption === "Schema"
            ? await fetchSchemaForTable(dataset)
            : null;
        const rows =
          selectedOption === "Rows" ? await fetchRowsForTable(dataset) : null;
        const name = getNameFromUrl(dataset.s3_url);
        const sql = await convertTextToSql({
          table: name,
          schema: schema,
          query: userQuery,
          rows: rows,
        });
        const sqlGenerated = sql.choices[0].message.content;
        setGeneratedSql(sqlGenerated);
        const sqlResult = await fetchSqlForQuery(
          sqlGenerated,
          dataset.extension,
        );
        setTableData(sqlResult);
      } else {
        const sqlResult = await fetchSqlForQuery(userQuery, dataset.extension);
        setTableData(sqlResult);
      }
      setLoading(false);
    } catch (error: any) {
      toast.error(error.code);
      setLoading(false);
    }
  };
  return (
    <div className="flex flex-col gap-6 mb-10">
      <div className="mt-2 flex justify-between w-full gap-6">
        <SearchBar
          inputClassName="!text-left py-3"
          wrapperClassName="w-full"
          withoutPrefixIcon={true}
          placeholder={
            textToSqlEnabled
              ? "write a query"
              : `Search SQL use ${getNameFromUrl(dataset.s3_url)} as table name`
          }
          callback={(value) => setUserQuery(value)}
        />
        <Button
          className="rounded-md bg-[#376789] text-white px-6 py-3"
          onClick={onSearch}
        >
          Query
        </Button>
      </div>
      <div className="flex justify-between">
        <div className="flex gap-6">
          <p>Search Results in:</p>
          <div className="flex gap-2">
            Schema
            <Switch
              checked={selectedOption === "Rows"}
              onCheckedChange={() =>
                setSelectedOption(selectedOption === "Rows" ? "Schema" : "Rows")
              }
            />
            Rows
          </div>
        </div>
        {textToSqlEnabled ? (
          <Button
            variant="link"
            className="px-2 flex gap-3"
            onClick={showGeneratedSql}
          >
            Generated with SQL
            {generatedSql ? (
              <Icons.ChevronDownIcon />
            ) : (
              <Icons.ChevronRightIcon size="large" />
            )}
          </Button>
        ) : (
          <p>{`dataset_id: ${getNameFromUrl(dataset.s3_url)}`}</p>
        )}
      </div>
      {generatedSqlVisible && (
        <div className="px-10 py-6 bg-white">
          {loading ? (
            <div className="flex justify-center items-center">
              <Loading />
              <div>Generating SQL...</div>
            </div>
          ) : generatedSql !== "" ? (
            generatedSql
          ) : (
            "No SQL generated yet! Please search for a query."
          )}
        </div>
      )}
      <SqlResultTable data={tableData} itemsPerPage={6} />
    </div>
  );
}

export default SearchSql;

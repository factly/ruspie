import React from "react";
import { useAppDataContext } from "./AppDataContext";

function GraphQL(){
  //getting the global context which has some getters and setters for getting the file name and schema
  let { currentFileSchema, currentFileName } = useAppDataContext();
  return (
    <>
      this is GraphQL tab
    </>
  )
}

export default GraphQL;
import React from "react";
import { useAppDataContext } from "./AppDataContext";

function Sql(){
  let { currentFileSchema, currentFileName } = useAppDataContext();
  return (
    <>
      this is sql tab
    </>
  )
}

export default Sql;
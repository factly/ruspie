import React, { useState } from "react";

let AppDataContext = React.createContext({});

export function useAppDataContext() {
  return React.useContext(AppDataContext);
}

export function AppContext({ children }) {
  // currentFileSchema has the schema of the file name entered by the user
  const [currentFileSchema, setCurrentFileSchema] = useState({});

  // fileName is the file name entered by the user
  const [currentFileName, setCurrentFileName] = useState("");

  let setSchema = (schemaData) => {
    setCurrentFileSchema(schemaData);
  };

  let setFileName = (fileName) => {
    setCurrentFileName(fileName);
  };

  let value = { currentFileName, currentFileSchema, setSchema, setFileName };

  return (
    <AppDataContext.Provider value={value}>{children}</AppDataContext.Provider>
  );
}

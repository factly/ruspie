export function isColumnDatatypeString(schema, columnName) {
  return (
    schema?.find((column) => column?.name === columnName)?.data_type === "Utf8"
  );
}

export const createFilter = (filterInputs, schema) => {
  let filterString = ``;
  filterInputs.forEach((element) => {
    if (isColumnDatatypeString(schema?.fields, element.columnName)) {
      filterString += `&filter[${element.columnName}]${element?.operator}'${element.value}'`;
    } else {
      filterString += `&filter[${element.columnName}]${element?.operator}${element.value}`;
    }
  });
  return filterString;
};

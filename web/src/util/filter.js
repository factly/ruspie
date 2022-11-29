export function isColumnDatatypeString(schema, columnName) {
  return schema?.find((column) => column?.name === columnName)?.data_type === 'Utf8';
}

export const createFilter = (filterInputs, schema) => {
  let filterString = ``;
  filterInputs.forEach((element) => {
    // if(isColumnDatatypeString(schema, element.columnName)){
    //   console.log('is string')
    //   filterString += `&filter[${element.columnName}]${element?.operator}'${element.value}'`
    // } else {
    //   console.log('is not string')

    //   filterString += `&filter[${element.columnName}]${element?.operator}${element.value}`
    //   console.log('this is numeric filter', filterString)
    // }
    filterString += `&filter[${element.columnName}]${element?.operator}'${element.value}'`;
  });
  // console.log('this is end filter string', filterString)
  return filterString;
};

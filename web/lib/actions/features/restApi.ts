type Filter = {
	columnName: string;
	operator: string;
	value: string;
};

export function isColumnDatatypeString(schema :any, columnName :string) {
  return schema?.find((column :any) => column?.name === columnName)?.data_type === 'Utf8';
}
function filterEmptyFilters(filters: Filter[]) {
  return filters.filter((filter) => {
    return (
      filter.columnName.trim() !== '' && filter.operator.trim() !== '' && filter.value.trim() !== ''
    );
  });
}

export const createFilter = (filters: Filter[], schema: any) => {
  const filterInputs = filterEmptyFilters(filters);
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

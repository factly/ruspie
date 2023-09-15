type TextToSqlParams = {
	table: string;
	schema: string | null;
	query: string | null;
	rows: string | null;
};



export const convertTextToSql = async ({ table, schema, query, rows }: TextToSqlParams) => {
  const headers = new Headers();
  headers.append('Content-Type', 'application/json');

  const body = JSON.stringify({
    prompt: `TEXT TO SQL CONVERSION
      TABLE_NAME: ${table}
      ${schema ? `TABLE_SCHEMA: ${schema}\n` : ''}
      ${rows ? `JSON_RESPONSE: ${rows}\n` : ''}
      NOTE: RETURN ONLY SQL, ALL THE KEYS IN THE JSON_RESPONSE ARE COLUMN NAMES , ALWAYS USE DOUBLE QUOTES FOR TABLE NAMES AND COLUMN NAMES IN SQL AND ALWAYS USE SINGLE QUOTES FOR VALUES IN SQL
      QUERY: ${query}
			EXAMPLE: SELECT * FROM "table_name" WHERE "column_name" = 'value'
    `,
  });

  const requestOptions :RequestInit= {
    method: 'POST',
    headers: headers,
    body: body,
    redirect: 'follow',
  };

  const response = await fetch(
    'https://text_to_sql_service.akshith-katkuri.workers.dev/',
    requestOptions,
  );

  if (!response.ok) {
    throw new Error(`Failed to convert text to SQL: ${response.status}`);
  }

  return response.json();
};

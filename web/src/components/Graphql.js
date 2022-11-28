import React, { useState } from "react";
import { useAppDataContext } from "./AppDataContext";
import ResponseTextArea from "./ResponseTextArea";
import './Util.css';
import './Rest.css';

function GraphQL(){
  // responseData stores the search data coming from ruspie after making the graphql request and displays it in the response textarea
  const [responseData, setResponseData] = useState('');
  // getting the global context which has some getters and setters for getting the file name and schema
  let { currentFileSchema, currentFileName } = useAppDataContext();


  return (
    <div className="request-response-div">
      <div className="request-division">
        <div className="rest-request-div">
          <h2>GraphQl Query</h2>
        </div>
        <div className="rest-form-div">
          <div className="form-input-div">
            <label htmlFor="file_format">
              File Format
            </label>
            <input name="file_format" type={'text'} id='file_format' placeholder="please enter file format" onChange={''} defaultValue={'csv'}/>
          </div>
        </div>
      </div>
      <ResponseTextArea response={responseData}/>
    </div>
  )
}

export default GraphQL;
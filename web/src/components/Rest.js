import React, { useState } from "react";
import ResponseTextArea from "./ResponseTextArea";
import './Rest.css'
import './Util.css'
import './LinkGroup.css';
import { useAppDataContext } from "./AppDataContext";
import Select from 'react-select'
import { Link } from "react-router-dom";

function Rest({  }){
  // responseData stores the search data coming from ruspie and displays it in the response textarea
  const [responseData, setResponseData] = useState('');

  // inputs stores the form data in key value pairs
  const [inputs, setInputs] = useState({
    limit : 20,
    file_format: 'csv',
    columns: []
  })
  const handleSubmit = (e) => {
    e.preventDefault();
    
    let queryParam = {}
    queryParam.limit = inputs.limit
    if (inputs.columns?.length > 0) {
      queryParam.columns = ''
      queryParam.columns = inputs.columns[0]
      for(let i=1; i < inputs.columns.length; i++){
        queryParam.columns += `,${inputs.columns[i]}`
      }
    }

    fetch(`http://localhost:8080/api/tables/${currentFileName}?` + new URLSearchParams(queryParam), 
    // {
    //   headers: {
    //     'FILE-EXT': inputs?.file_format
    //   }
    // }
    ) 
    .then((res) => {
      if (res.status !== 200) {
        return res.json().then(data => {
          throw Error(data?.message)
        })
      } else {
        return res.json()
      }
    }).then((responseData) => {
        setResponseData(JSON.stringify(responseData, null, 2));
    })
    .catch((err) => {
      alert(err.message)
    })

  }

  const handleChange = (event) => {
    const name = event.target.name;
    const value = event.target.value;
    setInputs(values => ({...values, [name]: value}))
  }

  const handleColumnsChange = (selectedValues) => {
    setInputs({...inputs, "columns": selectedValues?.map((object) => object?.value)})
  }
  //getting the global context which has some getters and setters for getting the file name and schema
  let { currentFileSchema, currentFileName } = useAppDataContext();
  return (
   <div className="request-response-div">
      <div className="request-division">
          <div className="rest-request-div">
            <h2>Request Parameters</h2>
            <div className="rest-form-div">
              <form onSubmit={handleSubmit} className='rest-form'>
                <div className="form-input-div">
                  <label htmlFor="file_format">
                    File Format
                  </label>
                  <input name="file_format" type={'text'} id='file_format' placeholder="please enter file format" onChange={handleChange} defaultValue={'csv'}/>
                </div>
                <div className="form-input-div">
                  <label htmlFor="limit" >
                    Page Size
                  </label>
                  <input name="limit" type={'number'} id='limit' placeholder="please enter limit" min={0} defaultValue={20}></input>
                </div>
                <div className="form-input-div">
                  <label htmlFor="columns">
                    Columns to Retrieve
                  </label>
                  <Select 
                    placeholder="please select columns to retrieve" 
                    multiple={true} 
                    isClearable={true}
                    onChange={handleColumnsChange}
                    isMulti={true}  
                    options={currentFileSchema?.fields?.map((field) => { 
                      return {
                        value: field.name,
                        label: field.name
                      }
                    })} 
                    className='attribute-Selector'
                  >
                  </Select>
                </div>
                <div className="form-input-div">
                  <label htmlFor="filters">
                    Filters
                  </label>
                  <input name="filters" type={'text'} id='filters' placeholder="please enter filters"></input>
                </div>
                <input type={'submit'} value='Execute Request'></input>
                <Link className='backLink' to="/"> 
                  Back to Schema 
                </Link>
              </form>
            </div>
          </div>
        </div> 
      <ResponseTextArea response={responseData}/>
    </div> 
  )
}

export default Rest;
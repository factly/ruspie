import React, { useState } from "react";
import './Rest.css'
function Rest({ attrList }){

  const [inputs, setInputs] = useState({})
  const handleSubmit = () => {
    console.log('submit called')
  }

  const handleChange = (event) => {
    const name = event.target.name;
    const value = event.target.value;
    setInputs(values => ({...values, [name]: value}))
  }

  return (
    <div className="rest-request-div">
      <h2>Request Parameters</h2>
      <div className="rest-form-div">
        <form onSubmit={handleSubmit} className='rest-form'>
          <div className="form-input-div">
            <label htmlFor="file_name" >
              File Name
            </label>
            <input type={'text'} id='file_name' placeholder="please enter file name" onChange={handleChange} required></input>
          </div>
          <div className="form-div-input">
            <label htmlFor="file_format">
              File Format
            </label>
            <input type={'text'} id='file_format' placeholder="please enter file format" onChange={handleChange} defaultValue={'csv'}/>
          </div>
          <div className="form-input-div">
            <label htmlFor="limit" >
              Limit
            </label>
            <input type={'number'} id='limit' placeholder="please enter limit" min={0} defaultValue={20}></input>
          </div>
          <div className="form-input-div">
            <label htmlFor="offset">
              Offset
            </label>
            <input type={'number'} id='offset' placeholder="please enter offset" min={0}></input>
          </div>
          <div className="form-input-div">
            <label htmlFor="columns">
              Columns to Retrieve
            </label>
            <input type={'text'} id='columns' placeholder="please enter columns to retrieve"></input>
          </div>
          <div className="form-input-div">
            <label htmlFor="filters">
              Filters
            </label>
            <input type={'text'} id='filters' placeholder="please enter filters"></input>
          </div>

          <input type={'submit'} value='Execute Request'></input>
        </form>
      </div>
    </div>
  )
}

export default Rest;
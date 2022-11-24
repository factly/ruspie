import React, { useState } from "react";
import './Rest.css'
function Schema({ handleResponseObject }){
  // file will hold the state of the file name entered by the user
  const [file, setFile] = useState('')

  // handle submit handle the form submission when the user clicks in Get Schema button
  const handleSubmit = (e) => {
    e.preventDefault();
    fetch(`http://localhost:8080/api/schema/${encodeURI(file)}`)
    .then((res) => {
      if (res.status !== 200) {
        return res.json().then(data => {
          throw Error(data?.message)
        })
      } else {
        return res.json()
      }
    }).then((responseData) => {
      handleResponseObject(JSON.stringify(responseData, undefined, 2))
    })
    .catch((err) => {
      alert(err.message)
    })
  }

  const handleChange = (e) =>{
    setFile(e.target.value)
  }
  
  return (
    <div className="rest-request-div">
      <h2>Get Schema</h2>
      <form onSubmit={(e) => handleSubmit(e)} className='rest-form'>
        <div className="form-input-div">
          <label htmlFor="file_name" >
            File Name
          </label>
          <input type={'text'} id='file_name' placeholder="please enter file name" onChange={handleChange} required></input>
          <input type={'submit'} value='Get Schema'></input>
        </div>
      </form>
    </div>
  )
}

export default Schema;
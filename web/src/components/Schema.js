import React, { useState } from 'react';
import './Util.css';
import './Rest.css';
import ResponseTextArea from './ResponseTextArea';
import { useAppDataContext } from './AppDataContext';
import LinkGroup from './LinkGroup';
import { schemaURL } from '../constants/apiEndpoints';

function Schema() {
  //getting the global context which has some getters and setters for getting the file name and schema
  let context = useAppDataContext();

  // responseData stores the schema data coming from ruspie api and displays it in the response textarea
  // const [responseData, setResponseData] = useState('' || (JSON.stringify(context?.currentFileSchema, null, 4)) !== '{}' ? JSON.stringify(context?.currentFileSchema): '');
  const [responseData, setResponseData] = useState('');

  // linkVisibility manages the visibility of the linkGroup
  const [linkVisibility, setLinkVisibility] = useState(false);

  // handle submit handle the form submission when the user clicks in Get Schema button
  const handleSubmit = (e) => {
    e.preventDefault();
    fetch(`${schemaURL}/${encodeURI(context.currentFileName)}`)
      .then((res) => {
        if (res.status !== 200) {
          return res.json().then((data) => {
            throw Error(data?.message);
          });
        } else {
          return res.json();
        }
      })
      .then((responseData) => {
        setResponseData(JSON.stringify(responseData, null, 2));
        context.setSchema(responseData);
        setLinkVisibility(true);
      })
      .catch((err) => {
        alert(err.message);
      });
  };

  // handleChange handles the change in the file_name input
  const handleChange = (e) => {
    context.setFileName(e.target.value);
  };

  React.useEffect(() => {
    if (context?.currentFileName !== '') {
      setLinkVisibility(true);
    }

    if (context?.currentFileSchema) {
      setResponseData(
        JSON.stringify(context?.currentFileSchema, null, 2) !== '{}'
          ? JSON.stringify(context?.currentFileSchema, null, 2)
          : '',
      );
    }
  }, []);
  return (
    <div className="request-response-div">
      <div className="request-division">
        <div className="rest-request-div">
          <h2>Get Schema</h2>
          <form onSubmit={(e) => handleSubmit(e)} className="rest-form">
            <div className="form-input-div">
              <label htmlFor="file_name">File Name</label>
              <input
                type={'text'}
                id="file_name"
                placeholder="please enter file name"
                onChange={handleChange}
                defaultValue={context?.currentFileName || ''}
                required
              ></input>
              <input type={'submit'} value="Get Schema"></input>
            </div>
          </form>
          {linkVisibility ? (
            <>
              <h3>Choose Method to Request Data</h3>
              <LinkGroup />
            </>
          ) : null}
        </div>
      </div>
      <ResponseTextArea response={responseData} />
    </div>
  );
}

export default Schema;

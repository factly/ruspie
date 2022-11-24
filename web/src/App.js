import React, { useState } from 'react';
import './App.css';
import GraphQL from './components/Graphql';
import Rest from './components/Rest';
import Schema from './components/Schema';
import Sql from './components/Sql';

function App() {
  const [tab, setTab] = useState(0) // tab - 0 is REST by default, 1 is SQL and 2 is GraphQL
  const handleTabClick = (tab) => {
    setTab(tab)
  }

  const handleResponseObjectFieldChange = (value) => {
    setResponseObject(value) 
  }

  //responseObject stores the response from ruspie
  const [responseObject, setResponseObject] = useState('');

  const [attrList, setAttrList] = useState([]);

  return (
    <div className="App">
      <div className='navbar'>
        <h1 id='heading'>Ruspie API Tool</h1>
      </div>
      <div className='header'>
        <ul className='header-list'>
          <li className='list-elements' onClick={() => handleTabClick(0)} style={(tab === 0) ? {borderBottom: "2px white solid"}: null}>Schema</li>
          <li className='list-elements' onClick={() => handleTabClick(3)} style={(tab === 3) ? {borderBottom: "2px white solid"}: null}>REST</li>
          <li className='list-elements' onClick={() => handleTabClick(1)} style={(tab === 1) ? {borderBottom: "2px white solid"}: null}>SQL</li>
          <li className='list-elements' onClick={() => handleTabClick(2)} style={(tab === 2) ? {borderBottom: "2px white solid"}: null}>GraphQL</li>
        </ul>
      </div>
      <div className='request-response-div'>
        <div className='request-division'>
          {
            (tab === 1) ? <Sql/> : (tab === 2) ? <GraphQL/> : (tab===3) ? <Rest attrList={attrList}/>: <Schema handleResponseObject={handleResponseObjectFieldChange}/>
          }
        </div>
        <div className='text-area-div'>
          <textarea id='response-textarea' placeholder='ruspie response...' value={responseObject}>
          </textarea>
        </div>
      </div>
    </div>
  );
}

export default App;

import React from 'react';
import './App.css';
import routes from './config/routes';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { AppContext } from './components/AppDataContext';

function App() {
  return (
    <AppContext>
      <div className="App">
        <div className='navbar'>
          <h1 id='heading'>Ruspie API Tool</h1>
        </div>
        <Router basename=''>
          <Routes>
            {
              routes.map((route) => <Route path={route.path} element={<route.Component/>}/>)
            }
          </Routes>
        </Router>
      </div>
    </AppContext>  
  );
}

export default App;

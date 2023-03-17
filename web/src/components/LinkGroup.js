import React from 'react';
import { Link } from 'react-router-dom';
import './LinkGroup.css';
import './Rest.css';

export function LinkGroup() {
  return (
    <div className="LinkDiv">
      <Link className="anchor" to="/rest">
        RestAPI
      </Link>
      <Link className="anchor" to="/graphql">
        GraphQL
      </Link>
      <Link className="anchor" to="/sql">
        SQL
      </Link>
    </div>
  );
}

export default LinkGroup;

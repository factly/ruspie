import React from "react";

function ResponseTextArea({ response }) {
  return (
    <div className="text-area-div">
      <textarea
        id="response-textarea"
        placeholder="ruspie response..."
        value={response}
      ></textarea>
    </div>
  );
}

export default ResponseTextArea;

import React, { useState } from "react";

function App() {
  const [isClicked, setClicked] = useState(false);

  return (
    <div>
      <div>Hello world</div>
      <button onClick={() => setClicked(!isClicked)}>
        Click {isClicked ? "ed" : ""}
      </button>
    </div>
  );
}

export default App;

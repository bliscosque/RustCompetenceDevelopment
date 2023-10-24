import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  const [note,setNote] = useState("Hello");

  async function rederMarkdown() {
    const response=await invoke("convert_markdown",{text: note});
    console.log(response);
  }

  return (
    <div className="m-2">
      
      <div className="flex justify-between items-center pb-2">
        <p>Editor</p>
        <div className="join">
          <button className="btn btn-sm join-item" onClick={async () => {await rederMarkdown()}}>MD to HTML</button>
        </div>
      </div>
      <textarea value={note} onChange={e => {setNote(e.target.value)}} className="w-full" rows={20} />
      <p>{note}</p>
    </div>
  );
}

export default App;

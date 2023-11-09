import { useEffect, useState } from "react";
import Editor from "./Editor";
import Database from "tauri-plugin-sql-api";


function App() {

  const [notes,setNotes]=useState([1]);

  async function createDB() {
    const loadedDB=await Database.load("sqlite:test.db");
    console.log(loadedDB);
  }

  async function loadNotes() {}

  useEffect(() => {
    createDB();
  }, [])

  return (
    <div className="bg-gray-700 h-screen p-2">
      <div className="flex flex-row justify-between items-center">
        <h1 className="text-white font-bold">All notes</h1>
        <button className="btn btn-sm" onClick={() => {
          setNotes([...notes,1]);
          console.log(notes)
        }}>Add notes</button>
      </div>
      <input className="my-2 w-full input"></input>
      {
        notes.map((item) => (
          <div className="flex flex-row justify-between items-center bg-green-200 border-green-500 my-2">
            <div className="cursor-pointer w-full h-full"><h2>Note</h2></div>
            <button className="btn btn-sm">Delete me</button>
          </div>
      ))}
    </div>
  )
}

export default App;
import { useEffect, useState } from "react";
import Editor from "./Editor";
import Database from "tauri-plugin-sql-api";


function App() {

  const [notes,setNotes]=useState([1]);

  async function createDB() {
    const loadedDB=await Database.load("sqlite:test.db");
    //const _first_load=await loadedDB.execute("CREATE TABLE IF NOT EXISTS notes (note_id CHAR NOT NULL PRIMARY KEY, note_text TEXT DEFAULT NULL);");
    //const result2=await loadedDB.execute("INSERT into notes(note_id,note_text) VALUES ($1, $2)", [crypto.randomUUID(), "DEMO"]);
    const result = await loadedDB.select("SELECT * FROM notes");
    console.log(result);
    setNotes(result);
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
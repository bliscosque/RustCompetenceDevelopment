import { useEffect, useState } from "react";
import Editor from "./Editor";
import Database from "tauri-plugin-sql-api";
import { addNoteDB, getSearch, removeNoteDB } from "./functions/db";


function App() {

  const [notes,setNotes]=useState([1]);
  const [db,setDB]=useState("");

  async function createDB() {
    const loadedDB=await Database.load("sqlite:test.db");
    //const _first_load=await loadedDB.execute("CREATE TABLE IF NOT EXISTS notes (note_id CHAR NOT NULL PRIMARY KEY, note_text TEXT DEFAULT NULL);");
    //const result2=await loadedDB.execute("INSERT into notes(note_id,note_text) VALUES ($1, $2)", [crypto.randomUUID(), "DEMO"]);
    
    setDB(loadedDB);
    loadNotes(loadedDB);

    
  }
  async function loadNotes(db) {
    const result = await db.select("SELECT * FROM notes");
    setNotes(result);
  }

  async function handleSearch(event) {
    const result=await getSearch(db,event.target.value);
    console.log(result)
    setNotes(result);
  }

  async function handleRemoveNote(uuid) {
    await removeNoteDB(db,uuid);
    await loadNotes(db);
  }

  async function addNote() {
    const newID=crypto.randomUUID();
    await addNoteDB(db,newID,"");
    await loadNotes(db);
  }

  useEffect(() => {
    createDB();
  }, [])

  return (
    <div className="bg-gray-700 h-screen p-2">
      <div className="flex flex-row justify-between items-center">
        <h1 className="text-white font-bold">All notes</h1>
        <button className="btn btn-sm" onClick={() => {addNote()}}>Add notes</button>
      </div>
      <input onChange={async e => {handleSearch(e)}} className="my-2 w-full input"></input>
      {
        notes.map((item) => (
          <div key={item.note_id} className="px-2 flex flex-row justify-between items-center bg-green-200 border-green-500 my-2">
            <div className="cursor-pointer w-full h-full"><h2>{item.note_text}</h2></div>
            <button onClick={async () => {await handleRemoveNote(item.note_id)}} className="btn btn-sm">Delete me</button>
          </div>
      ))}
    </div>
  )
}

export default App;
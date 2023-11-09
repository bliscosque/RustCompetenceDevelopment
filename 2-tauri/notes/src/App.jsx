import Editor from "./Editor";


function App() {
  return (
    <div className="bg-gray-700 h-screen p-2">
      <div className="flex flex-row justify-between items-center">
        <h1 className="text-white font-bold">All notes</h1>
        <button className="btn btn-sm">Add notes</button>
      </div>
      <input className="my-2 w-full input"></input>
      <div className="flex flex-row justify-between items-center bg-green-200 border-green-500 my-2">
        <div className="cursor-pointer w-full h-full"><h2>Note</h2></div>
        <button className="btn btn-sm">Delete me</button>
      </div>
    </div>
  )
}

export default App;
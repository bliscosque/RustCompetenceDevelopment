import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {writeText} from "@tauri-apps/api/clipboard"
import "./App.css";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification
} from "@tauri-apps/api/notification"
import { tauri } from "@tauri-apps/api";

function Editor() {

  const [note,setNote] = useState("Hello");
  const [isRendered,setRender] = useState(false);
  const [markdownHTML, setMarkdownHTML] = useState("");

  async function renderMarkdown() {
    if (!isRendered) {
      const response=await invoke("convert_markdown",{text: note});
      setMarkdownHTML({__html:response});
    }
    setRender(!isRendered);
  }

  return (
    <div className="m-2">
      
      <div className="flex justify-between items-center pb-2">
        <h1>Editor</h1>
        <div className="join">
          <label className="btn btn-sm join-item swap">
            <input onChange={async () => {await renderMarkdown()}} type="checkbox"></input>
            <div className="swap-on">HTML</div>
            <div className="swap-off">MD</div>
          </label>
          <button className="btn btn-sm join-item" onClick={ async () => {
            await writeText(note);
            let permissionGranted = await isPermissionGranted()
            if (!isPermissionGranted) {
              const permission = await requestPermission();
              permissionGranted = permission==="granted";
            }
            if (isPermissionGranted) {
              sendNotification({title: "notes", body: "copy txt"})
            }
          }}>Copy</button>
        </div>
      </div>
      {isRendered?
      <div className="prose" dangerouslySetInnerHTML={markdownHTML}></div>
      :
      <textarea value={note} onChange={e => {setNote(e.target.value)}} className="w-full" rows={20} />
      }
      <p>{note}</p>
    </div>
  );
}

export default Editor;

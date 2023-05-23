import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [folderCount, setFolderCount] = useState(0);
  const [open, setOpen] = useState(false);
  const [images, setImages] = useState([]);
  const openCam = async () => {
    setOpen(true);
    const res = await invoke("start_threads");
    console.log(res);
  };
  const closeCam = async () => {
    setOpen(false);
    await invoke("stop_thread");
  };

  function getArrayRangeBasedOnFolderCount() {
    let arr = [];
    let count = 1;
    while (count <= folderCount) {
      arr.push(count);
      count += 1;
    }
    return arr;
  }
  async function getImages() {
    const res = await invoke("get_images");
    setImages(res);
  }

  useEffect(() => {
    invoke("get_folder_count")
      .then((res) => {
        setFolderCount(res);
      })
      .catch((err) => {
        console.log(err);
      });
  }, [images.length]);

  useEffect(() => {
    const interval = setInterval(() => {
      invoke("get_images", { dirNumber: "1" })
        .then((res) => {
          setImages(res);
        })
        .catch((err) => {
          console.log(err);
        });
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="h-[100vh] w-full overflow-hidden px-20">
      <div className="w-full mt-8">
        <div className="w-full flex flex-row gap-4 mb-8">
          <button
            style={{
              padding: "4px 8px",
              backgroundColor: "white",
              color: "black",
            }}
            onClick={() => {
              if (!open) {
                openCam();
              } else {
                closeCam();
              }
            }}
          >
            {open ? "Stop" : "Start"}
          </button>
          <button
            style={{
              padding: "4px 8px",
              backgroundColor: "white",
              color: "black",
            }}
            onClick={getImages}
          >
            Get Images
          </button>
        </div>
        <div className="flex flex-row gap-4 mb-4">
          {getArrayRangeBasedOnFolderCount().map((f) => (
            <button key={f}>Open Folder {f}</button>
          ))}
        </div>

        <div className="flex flex-row gap-2 h-[70vh] flex-wrap justify-center overflow-y-scroll mb-12 border-2 border-white rounded-lg py-8">
          {images.map((image, i) => (
            <img
              className="w-[20%] rounded-md"
              key={i}
              src={`../images/${image}`}
              alt=""
            />
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;

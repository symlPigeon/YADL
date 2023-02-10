const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("lyricsapi", {
    resizeLyricsWindow: (height) => {
        console.log("resizeLyricsWindow" + height);
        ipcRenderer.send("resizeLyricsWindow", height);
    },
});

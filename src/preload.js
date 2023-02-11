const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("lyricsapi", {
    resizeLyricsWindow: (height, width) => {
        console.log("resizeLyricsWindow" + height + "x" + width);
        ipcRenderer.send("resizeLyricsWindow", height, width);
    },
    toggleFixedWindow: (status) => {
        console.log("toggleFixedWindow" + status);
        ipcRenderer.send("toggleFixedWindow", status);
    },
    reportMouseEvents: (status) => {
        if (status) {
            // mouse enter
            console.log("repoer mouse enter");
            ipcRenderer.send("mouseEvents", true);
        } else {
            // mouse leave
            console.log("report mouse leave");
            ipcRenderer.send("mouseEvents", false);
        }
    },
    toggleButtonStatus: (callback, status) => {
        console.log("toggleButtonStatus" + status);
        ipcRenderer.on("toggleButtonStatus", callback);
    },
});

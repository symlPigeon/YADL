const { BrowserWindow, app, ipcMain } = require("electron");
const path = require("path");

const createWindow = () => {
    const win = new BrowserWindow({
        height: 600,
        width: 800,
        //transparent: true,
        //frame: false
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("public/index.html");
    win.setAlwaysOnTop(true, "floating");
    //win.removeMenu();
    ipcMain.handle("resizeLyricsWindow", (height) => {
        win.setSize(win.width, height);
    });
};

app.on("ready", () => {
    console.log("App is ready");
    createWindow();
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

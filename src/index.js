const { BrowserWindow, app, ipcMain } = require("electron");
const path = require("path");
let pinned_status = false;

const createWindow = () => {
    const win = new BrowserWindow({
        height: 200,
        width: 800,
        transparent: true,
        frame: false,
        maximizable: false,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("public/index.html");
    win.setAlwaysOnTop(true, "floating");
    win.removeMenu();

    // 设置窗口大小
    ipcMain.on("resizeLyricsWindow", (event, height, width) => {
        const webContents = event.sender;
        const win = BrowserWindow.fromWebContents(webContents);
        console.log("resize window to " + height + "x" + width);
        win.setSize(width, height);
    });
    // 设置窗口鼠标穿透状态
    ipcMain.on("toggleFixedWindow", (event, status) => {
        console.log("toggleFixedWindow " + status);
        pinned_status = status;
    });
    // 处理窗口鼠标穿透
    ipcMain.on("mouseEvents", (event, status) => {
        const webContents = event.sender;
        const win = BrowserWindow.fromWebContents(webContents);
        console.log("mouseEvents:" + status);
        console.log("pinned status:" + pinned_status);
        if (pinned_status) {
            // 如果pinned，只处理按钮事件
            if (status) {
                // 鼠标进入了按钮区域
                win.setIgnoreMouseEvents(false);
            } else {
                // 鼠标不在按钮区域内
                win.setIgnoreMouseEvents(true, { forward: true });
            }
        } else {
            // 放过所有事件
            win.setIgnoreMouseEvents(false);
        }
    });
};

app.on("ready", () => {
    console.log("App is ready");
    createWindow();
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

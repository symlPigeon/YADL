const { BrowserWindow, app, ipcMain } = require("electron");
const path = require("path");
let pinned_status = false;
let mainWindow = null;
let floatWindow = null;

const createMainWindow = () => {
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
    mainWindow = win;
};

const createFloatWindow = () => {
    const win = new BrowserWindow({
        height: 60,
        width: 60,
        transparent: true,
        frame: false,
        maximizable: false,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("public/float.html");
    win.setAlwaysOnTop(true, "floating");
    win.removeMenu();
    floatWindow = win;
};

const moveWindow = () => {
    // 把浮动窗口塞到顶层
    mainWindow.moveTop();
    floatWindow.moveTop();
    // 浮动窗口跟随主窗口移动
    floatWindow.setPosition(
        mainWindow.getPosition()[0],
        mainWindow.getPosition()[1]
    );
};

// 设置窗口大小
ipcMain.on("resizeLyricsWindow", (event, height, width) => {
    const webContents = event.sender;
    const win = BrowserWindow.fromWebContents(webContents);
    console.log("resize window to " + height + "x" + width);
    win.setSize(width, height);
    // 浮动窗口大小需要根据主窗口设置
    floatWindow.setSize(60, 60);
    floatWindow.setPosition(win.getPosition()[0], win.getPosition()[1]);
});

// 设置窗口鼠标穿透状态
ipcMain.on("toggleFixedWindow", (event, status) => {
    console.log("toggleFixedWindow " + status);
    pinned_status = status;
    mainWindow.webContents.send("toggleButtonStatus", pinned_status);
    mainWindow.setIgnoreMouseEvents(pinned_status);
});

// 对悬浮窗口处理窗口鼠标穿透
ipcMain.on("mouseEvents", (event, status) => {
    const webContents = event.sender;
    const win = BrowserWindow.fromWebContents(webContents);
    console.log("mouseEvents:" + status);
    console.log("pinned status:" + pinned_status);
    if (status) {
        // 鼠标进入了按钮区域
        win.setIgnoreMouseEvents(false);
    } else {
        // 鼠标不在按钮区域内
        win.setIgnoreMouseEvents(true);
    }
});

app.on("ready", () => {
    console.log("App is ready");
    createMainWindow();
    createFloatWindow();
    mainWindow.addListener("move", moveWindow);
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

const { app, BrowserWindow } = require('electron')

function createWindow() {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        frame: false
    });

    win.fullScreen = true;
    win.loadFile("tetris-client.html");
}

app.whenReady().then(() => {
    createWindow();
})
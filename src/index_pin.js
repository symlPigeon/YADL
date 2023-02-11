function registerClickThroughEvent() {
    // 给按钮加上点击穿透事件，禁用本界面的按钮
    // 防止说就是悬浮歌词界面飘在实际按钮界面上面……
    var ignoreClickEle = document.getElementById("ignore-click");
    ignoreClickEle.addEventListener("mouseenter", () => {
        lyricsapi.reportMouseEvents(false);
    });
    ignoreClickEle.addEventListener("mouseleave", () => {
        lyricsapi.reportMouseEvents(true);
    });
}

function registerButtonChangeEvent() {
    // 悬浮歌词界面的按钮应该跟随跟随锁定状态
    var changedButton = document.getElementById("ignore-click");
    window.lyricsapi.toggleButtonStatus((event, status) => {
        console.log("Change button status: ", status);
        if (status) {
            changedButton.innerHTML =
                '<img src="../assets/unpin.svg" class="pin">';
        } else {
            changedButton.innerHTML =
                '<img src="../assets/pin.svg" class="pin">';
        }
    });
}

registerClickThroughEvent();
registerButtonChangeEvent();

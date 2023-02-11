let pinned_status = false;

// const clickThroughEles = document.getElementsByClassName("click-through");

// for (i = 0; i < clickThroughEles.length; i++) {
//     clickThroughEles[i].addEventListener("mouseenter", () => {
//         console.log("reportMouseEvents true");
//         lyricsapi.reportMouseEvents(true);
//     });
//     clickThroughEles[i].addEventListener("mouseleave", () => {
//         console.log("reportMouseEvents false");
//         lyricsapi.reportMouseEvents(false);
//     });
// }

const button_pinned = document.getElementById("toggle-pinned");

button_pinned.addEventListener("click", () => {
    pinned_status = !pinned_status;
    console.log("toggleFixedWindow" + pinned_status);
    // 不必要修改按钮了，反正也看不到
    // if (pinned_status) {
    //     button_pinned.innerHTML =
    //         '<img src="../assets/unpin.svg" class="pin" style="visibility:hidden">';
    // } else {
    //     button_pinned.innerHTML =
    //         '<img src="../assets/pin.svg" class="pin" style="visibility:hidden">';
    // }
    lyricsapi.toggleFixedWindow(pinned_status); // just send signal here
});

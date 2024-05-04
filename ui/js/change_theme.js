window.addEventListener("DOMContentLoaded", () => {
    const { listen } = window.__TAURI__.event;

    // Initial theme...
    lyrics = document.getElementById("lyric");
    lyrics.className = "lyric light-blue";

    listen("menu_set_theme", (e) => {
        context = e.payload;
        lyrics = document.getElementById("lyric");
        switch (context) {
            case "light_blue":
                lyrics.className = "lyric light-blue";
                console.log("changing to light blue");
                break;
            case "green":
                lyrics.className = "lyric green";
                break;
            case "yellow":
                lyrics.className = "lyric yellow";
                break;
            case "red":
                lyrics.className = "lyric red";
                break;
            default:
                break;
        }
    });
});

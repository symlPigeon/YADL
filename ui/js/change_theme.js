window.addEventListener("DOMContentLoaded", async () => {
    const { listen } = window.__TAURI__.event;
    const { invoke } = window.__TAURI__.tauri;

    // Initial theme...
    lyrics = document.getElementById("lyric");
    await invoke("get_init_theme", {}).then((theme) => {
        lyrics.className = "lyric " + theme || "light-blue";
    });

    listen("menu_set_theme", (e) => {
        context = e.payload;
        lyrics = document.getElementById("lyric");
        let is_valid_context = true;
        switch (context) {
            case "light-blue":
                lyrics.className = "lyric light-blue";
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
                is_valid_context = false;
                break;
        }
        if (is_valid_context) {
            invoke("change_window_theme", { theme: context });
        }
    });
});

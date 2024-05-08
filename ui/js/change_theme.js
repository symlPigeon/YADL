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
            case "custom-theme":
                lyrics.className = "lyric custom-theme";
                break;
            default:
                is_valid_context = false;
                break;
        }
        if (is_valid_context) {
            invoke("change_window_theme", { theme: context });
        }
    });

    function modifyCSSRule(styleId, color, shadow) {
        const styleElement = document.getElementById(styleId);
        const styleSheet = styleElement.sheet;

        for (let i = 0; i < styleSheet.cssRules.length; i++) {
            const rule = styleSheet.cssRules[i];
            if (rule.selectorText === ".custom-theme") {
                rule.style.color = color;
                rule.style.textShadow = `${shadow} 1px 0 5px`;
                break;
            }
        }
    }

    listen("modify_custom_settings", (e) => {
        context = e.payload;
        text_color = context[0];
        text_shadow = context[1];

        modifyCSSRule("theme-style", text_color, text_shadow);
    });

    await invoke("init_custom_theme", {});
});
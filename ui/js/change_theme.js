window.addEventListener("DOMContentLoaded", () => {
    const { listen } = window.__TAURI__.event;

    listen("menu_set_theme", (e) => {
        context = e.payload;
        switch (context) {
            case "light_blue":
                break;
            case "green":
                break;
            case "yellow":
                break;
            case "red":
                break;
            default:
                break;
        }
    });
});

window.addEventListener("DOMContentLoaded", () => {
    const { invoke } = window.__TAURI__.tauri;
    const { listen } = window.__TAURI__.event;

    listen("menu_toggle_pause_resume", (e) => {
        invoke("toggle_pause_resume", {});
    });
    listen("menu_toggle_pinned", (e) => {
        invoke("toggle_window_pinned", {});
    });
    listen("menu_exit_app", async (e) => {
        await invoke("exit_app", {});
    });
    listen("menu_start_custom_settings", async (e) => {
        await invoke("start_custom_settings", {});
    });

    window.addEventListener("contextmenu", async (event) => {
        event.preventDefault();

        // get current theme
        const theme = document.getElementById("lyric").classList[1];

        invoke("plugin:context_menu|show_context_menu", {
            items: [
                {
                    label: "Pause / Resume",
                    event: "menu_toggle_pause_resume",
                },
                {
                    label: "Themes",
                    subitems: [
                        {
                            label: "Light Blue",
                            event: "menu_set_theme",
                            payload: "light-blue",
                            checked: theme === "light-blue",
                        },
                        {
                            label: "Green",
                            event: "menu_set_theme",
                            payload: "green",
                            checked: theme === "green",
                        },
                        {
                            label: "Yellow",
                            event: "menu_set_theme",
                            payload: "yellow",
                            checked: theme === "yellow",
                        },
                        {
                            label: "Red",
                            event: "menu_set_theme",
                            payload: "red",
                            checked: theme === "red",
                        },
                        {
                            label: "Custom Color",
                            event: "menu_set_theme",
                            payload: "custom-theme",
                            checked: theme === "custom-theme",
                        },
                        {
                            label: "",
                            is_separator: true,
                        },
                        {
                            label: "Custom Settings",
                            event: "menu_start_custom_settings",
                        },
                    ],
                },
                {
                    label: "Lock Lyrics",
                    event: "menu_toggle_pinned",
                },
                {
                    label: "Exit App",
                    event: "menu_exit_app",
                },
            ],
        });
        invoke("reset_pin_window_focus");
    });
});

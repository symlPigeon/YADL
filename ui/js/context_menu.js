window.addEventListener("DOMContentLoaded", () => {
    const { invoke } = window.__TAURI__.tauri;
    const { listen } = window.__TAURI__.event;

    listen("menu_toggle_pause_resume", (e) => {
        invoke("toggle_pause_resume", {});
    });
    listen("menu_toggle_pinned", (e) => {
        invoke("toggle_window_pinned", {});
    });
    listen("menu_exit_app", (e) => {
        invoke("exit_app", {});
    });

    window.addEventListener("contextmenu", async (event) => {
        event.preventDefault();

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
                            label: "Not Done Yet :(",
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
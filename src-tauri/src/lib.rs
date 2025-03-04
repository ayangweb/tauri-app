use tauri::{ActivationPolicy, Manager};
use tauri_nspanel::{cocoa::appkit::NSWindowCollectionBehavior, panel_delegate, WebviewWindowExt};

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
#[allow(non_upper_case_globals)]
const NSResizableWindowMask: i32 = 1 << 3;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();

            app.set_activation_policy(ActivationPolicy::Accessory);

            let panel = main_window.to_panel().unwrap();

            panel.set_level(20);

            panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel | NSResizableWindowMask);

            panel.set_collection_behaviour(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
            );

            let delegate = panel_delegate!(EcoPanelDelegate {
                window_did_become_key,
                window_did_resign_key,
                window_did_resize,
                window_did_move
            });

            panel.set_delegate(delegate);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

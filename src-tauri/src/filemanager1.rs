// Interface D-Bus standard `org.freedesktop.FileManager1` : permet aux navigateurs et apps tierces
// (« Afficher dans le dossier », « Ouvrir le dossier contenant ») de cibler Vela au lieu de Nemo/Dolphin.
// Chaque méthode réémet l'event `vela-control` déjà géré par le front + ramène la fenêtre au premier plan.
use percent_encoding::percent_decode_str;
use tauri::{AppHandle, Emitter, Manager};

const BUS_NAME: &str = "org.freedesktop.FileManager1";
const OBJ_PATH: &str = "/org/freedesktop/FileManager1";

struct FileManager1 {
    app: AppHandle,
}

#[zbus::interface(name = "org.freedesktop.FileManager1")]
impl FileManager1 {
    // Ouvre un (ou plusieurs) dossiers : on navigue dans le premier.
    fn show_folders(&self, uris: Vec<String>, _startup_id: String) {
        if let Some(path) = first_path(&uris) {
            self.dispatch("navigate", &path);
        }
    }

    // Révèle des items (fichiers/dossiers) dans leur dossier parent en les sélectionnant.
    fn show_items(&self, uris: Vec<String>, _startup_id: String) {
        if let Some(path) = first_path(&uris) {
            self.dispatch("reveal_in_folder", &path);
        }
    }

    // Pas de modale propriétés via D-Bus : on se contente de révéler l'item.
    fn show_item_properties(&self, uris: Vec<String>, _startup_id: String) {
        if let Some(path) = first_path(&uris) {
            self.dispatch("reveal_in_folder", &path);
        }
    }
}

impl FileManager1 {
    // Émet l'event control vers le front puis ramène la fenêtre principale au premier plan.
    fn dispatch(&self, action: &str, path: &str) {
        let payload = serde_json::json!({ "action": action, "args": { "path": path } });
        if let Err(e) = self.app.emit("vela-control", payload) {
            eprintln!("[filemanager1] emit {action} : {e}");
        }
        if let Some(win) = self.app.get_webview_window("main") {
            let _ = win.unminimize();
            let _ = win.set_focus();
        }
    }
}

// Convertit la première URI `file://` en chemin local décodé (percent-decoding). Ignore les autres schémas.
fn first_path(uris: &[String]) -> Option<String> {
    let uri = uris.first()?;
    let after = uri.strip_prefix("file://")?;
    // `file:///chemin` → "/chemin" ; `file://host/chemin` → on coupe au premier '/'.
    let raw = if after.starts_with('/') {
        after
    } else {
        let idx = after.find('/')?;
        &after[idx..]
    };
    let decoded = percent_decode_str(raw).decode_utf8_lossy().to_string();
    (!decoded.is_empty()).then_some(decoded)
}

// Démarre le service D-Bus dans un thread dédié. Best-effort : si le nom est déjà détenu
// (Nemo/Dolphin actif) ou si le bus de session est absent, on log et Vela continue sans.
pub fn init(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || match serve(app) {
        Ok(_conn) => loop {
            // Garde la connexion vivante : zbus traite les appels sur ses threads internes.
            std::thread::park();
        },
        Err(e) => eprintln!("[filemanager1] service D-Bus non démarré : {e}"),
    });
}

fn serve(app: AppHandle) -> Result<zbus::blocking::Connection, zbus::Error> {
    zbus::blocking::connection::Builder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJ_PATH, FileManager1 { app })?
        .build()
}

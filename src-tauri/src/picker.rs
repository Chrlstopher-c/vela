// Mode sélecteur : Vela lancé par le portail de fichiers (xdg-desktop-portal-termfilechooser) pour choisir un ou
// plusieurs fichiers, un dossier ou un emplacement d'enregistrement ; les chemins choisis sont écrits dans le fichier
// de sortie du portail, une ligne par chemin, puis Vela se ferme.
use serde::Serialize;
use std::sync::Mutex;

#[derive(Clone, Serialize)]
pub struct PickerConfig {
    /// open | multiple | directory | save
    pub mode: String,
    /// dossier de départ, ou fichier proposé à l'enregistrement
    pub path: String,
    #[serde(skip)]
    pub out: String,
}

pub struct PickerState(pub Mutex<Option<PickerConfig>>);

/// `vela --pick <mode> <chemin> <sortie>`
pub fn from_args() -> Option<PickerConfig> {
    let args: Vec<String> = std::env::args().collect();
    let i = args.iter().position(|a| a == "--pick")?;
    match (args.get(i + 1), args.get(i + 2), args.get(i + 3)) {
        (Some(mode), Some(path), Some(out)) => Some(PickerConfig {
            mode: mode.clone(),
            path: path.clone(),
            out: out.clone(),
        }),
        _ => {
            eprintln!("[picker] usage : --pick <open|multiple|directory|save> <chemin> <sortie>");
            None
        }
    }
}

#[tauri::command]
pub fn picker_config(state: tauri::State<PickerState>) -> Option<PickerConfig> {
    state.0.lock().ok().and_then(|c| c.clone())
}

/// Écrit la réponse (vide = annulé) et ferme Vela.
#[tauri::command]
pub fn picker_finish(app: tauri::AppHandle, state: tauri::State<PickerState>, paths: Vec<String>) -> Result<(), String> {
    let out = state.0.lock().map_err(|e| e.to_string())?.as_ref().map(|c| c.out.clone());
    if let Some(out) = out {
        let body = if paths.is_empty() { String::new() } else { paths.join("\n") + "\n" };
        if let Err(e) = std::fs::write(&out, body) {
            eprintln!("[picker] écriture de {out} impossible : {e}");
            return Err(e.to_string());
        }
    }
    app.exit(0);
    Ok(())
}

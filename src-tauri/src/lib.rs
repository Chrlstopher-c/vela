// Point d'entrée Tauri : enregistre les commandes natives exposées au front + le control plane MCP.
// Intégration dans un autre hôte (Echo OS) : `gestionnaire_commandes`, `est_commande_vela`, `preparer_integre`.
mod actions;
mod advsearch;
mod analyze;
mod apps;
mod archive;
mod browser;
mod audio;
mod control;
mod convert;
mod dircmp;
mod disk;
mod translate;
mod codeindex;
mod download_job;
mod downloader;
mod favorites;
mod filemanager1;
mod fs_ops;
mod gallery;
mod picker;
mod git;
mod integrity;
mod index;
mod imaging;
mod media_probe;
mod ocr;
mod ops;
mod pdf;
mod places;
mod project;
mod replace;
mod player;
mod profiles;
mod share;
mod stems;
mod tags;
mod templates;
mod terminal;
mod thumbs;
mod video;
mod watcher;

// Chemin de dossier passé au démarrage (association MIME inode/directory via .desktop %U),
// récupéré une seule fois par le front au montage pour y naviguer.
struct PendingPath(std::sync::Mutex<Option<String>>);

#[tauri::command]
fn take_pending_path(state: tauri::State<PendingPath>) -> Option<String> {
    state.0.lock().ok().and_then(|mut g| g.take())
}


/// Noms des commandes de `gestionnaire_commandes`, dans le même ordre (un test vérifie la concordance).
const NOMS_COMMANDES: &[&str] = &[
    "picker_config",
    "picker_finish",
    "list_dir",
    "read_file",
    "read_file_chunk",
    "read_byte_range",
    "write_file_base64",
    "write_file",
    "rename_entry",
    "delete_entry",
    "create_dir",
    "search_dir",
    "read_file_base64",
    "open_native",
    "move_entry",
    "get_entry_props",
    "take_pending_path",
    "trash_entries",
    "delete_entries",
    "copy_entries",
    "move_entries",
    "scan_conflicts",
    "transfer_pause",
    "transfer_resume",
    "transfer_cancel",
    "start_compression",
    "translate_capabilities",
    "translate_text",
    "translate_install_lang",
    "translate_file",
    "codeindex_available",
    "codeindex_search",
    "codeindex_index",
    "search_content",
    "trash_dir",
    "trash_count",
    "empty_trash",
    "restore_trash",
    "watch_dir",
    "thumbnail",
    "available_shells",
    "term_open",
    "term_input",
    "term_resize",
    "term_close",
    "term_resolve",
    "home_dir",
    "list_places",
    "load_favorites",
    "save_favorites",
    "load_profiles",
    "save_profiles",
    "browser_create",
    "browser_navigate",
    "browser_show",
    "browser_hide",
    "browser_eval",
    "browser_close",
    "browser_reset",
    "load_tags",
    "set_tag",
    "template_list",
    "template_instantiate",
    "save_as_template",
    "share_start",
    "share_stop",
    "share_active",
    "pdf_merge",
    "pdf_extract_pages",
    "pdf_rotate",
    "project_detect",
    "search_replace_preview",
    "search_replace_apply",
    "search_advanced",
    "image_exif",
    "image_palette",
    "media_dimensions",
    "images_batch",
    "video_to_gif",
    "video_subtitles",
    "video_target_size",
    "video_storyboard",
    "analyze_disk",
    "compare_dirs",
    "player_open",
    "player_open_audio",
    "player_position",
    "player_pause",
    "player_resume",
    "player_seek",
    "player_set_volume",
    "player_close",
    "list_archive",
    "start_extraction",
    "extraction_pause",
    "extraction_resume",
    "extraction_cancel",
    "extraction_provide_password",
    "get_apps_for_file",
    "search_path_bins",
    "set_default_app",
    "set_custom_command",
    "media_capabilities",
    "media_probe",
    "download_capabilities",
    "download_probe",
    "download_start",
    "download_cancel",
    "youtube_auth_status",
    "audio_trim",
    "audio_fade",
    "audio_normalize",
    "audio_convert",
    "audio_remove_vocals",
    "image_crop",
    "image_rotate",
    "image_flip",
    "image_resize",
    "image_adjust",
    "image_convert",
    "image_apply_ops",
    "video_trim",
    "video_extract_frame",
    "video_extract_audio",
    "video_convert",
    "video_convert_cancel",
    "stems_status",
    "stems_separate",
    "stems_install",
    "stems_cancel",
    "convert_capabilities",
    "convert_targets",
    "convert_file",
    "images_to_pdf",
    "merge_csv",
    "organize_dir",
    "git_repo_root",
    "git_status",
    "git_current_branch",
    "git_ahead_behind",
    "git_branches",
    "git_log",
    "git_stage",
    "git_unstage",
    "git_commit",
    "git_checkout_branch",
    "git_diff_file",
    "disk_free",
    "path_exists",
    "file_hash",
    "file_kind",
    "index_refresh",
    "global_search",
    "ocr_capabilities",
    "ocr_extract",
];

/// Vrai si `nom` est une commande exposée par Vela.
pub fn est_commande_vela(nom: &str) -> bool {
    NOMS_COMMANDES.contains(&nom)
}

/// Gestionnaire IPC de toutes les commandes de Vela, à passer à `Builder::invoke_handler`.
pub fn gestionnaire_commandes() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        picker::picker_config,
        picker::picker_finish,
        fs_ops::list_dir,
        fs_ops::read_file,
        fs_ops::read_file_chunk,
        fs_ops::read_byte_range,
        fs_ops::write_file_base64,
        fs_ops::write_file,
        fs_ops::rename_entry,
        fs_ops::delete_entry,
        fs_ops::create_dir,
        fs_ops::search_dir,
        fs_ops::read_file_base64,
        fs_ops::open_native,
        fs_ops::move_entry,
        fs_ops::get_entry_props,
        take_pending_path,
        ops::trash_entries,
        ops::delete_entries,
        ops::copy_entries,
        ops::move_entries,
        ops::scan_conflicts,
        ops::transfer_pause,
        ops::transfer_resume,
        ops::transfer_cancel,
        archive::start_compression,
        translate::translate_capabilities,
        translate::translate_text,
        translate::translate_install_lang,
        translate::translate_file,
        codeindex::codeindex_available,
        codeindex::codeindex_search,
        codeindex::codeindex_index,
        ops::search_content,
        ops::trash_dir,
        ops::trash_count,
        ops::empty_trash,
        ops::restore_trash,
        watcher::watch_dir,
        thumbs::thumbnail,
        terminal::available_shells,
        terminal::term_open,
        terminal::term_input,
        terminal::term_resize,
        terminal::term_close,
        terminal::term_resolve,
        places::home_dir,
        places::list_places,
        favorites::load_favorites,
        favorites::save_favorites,
        profiles::load_profiles,
        profiles::save_profiles,
        browser::browser_create,
        browser::browser_navigate,
        browser::browser_show,
        browser::browser_hide,
        browser::browser_eval,
        browser::browser_close,
        browser::browser_reset,
        tags::load_tags,
        tags::set_tag,
        templates::template_list,
        templates::template_instantiate,
        templates::save_as_template,
        share::share_start,
        share::share_stop,
        share::share_active,
        pdf::pdf_merge,
        pdf::pdf_extract_pages,
        pdf::pdf_rotate,
        project::project_detect,
        replace::search_replace_preview,
        replace::search_replace_apply,
        advsearch::search_advanced,
        gallery::image_exif,
        gallery::image_palette,
        gallery::media_dimensions,
        imaging::images_batch,
        video::video_to_gif,
        video::video_subtitles,
        video::video_target_size,
        video::video_storyboard,
        analyze::analyze_disk,
        dircmp::compare_dirs,
        player::player_open,
        player::player_open_audio,
        player::player_position,
        player::player_pause,
        player::player_resume,
        player::player_seek,
        player::player_set_volume,
        player::player_close,
        archive::list_archive,
        archive::start_extraction,
        archive::extraction_pause,
        archive::extraction_resume,
        archive::extraction_cancel,
        archive::extraction_provide_password,
        apps::get_apps_for_file,
        apps::search_path_bins,
        apps::set_default_app,
        apps::set_custom_command,
        media_probe::media_capabilities,
        media_probe::media_probe,
        downloader::download_capabilities,
        downloader::download_probe,
        download_job::download_start,
        download_job::download_cancel,
        download_job::youtube_auth_status,
        audio::audio_trim,
        audio::audio_fade,
        audio::audio_normalize,
        audio::audio_convert,
        audio::audio_remove_vocals,
        imaging::image_crop,
        imaging::image_rotate,
        imaging::image_flip,
        imaging::image_resize,
        imaging::image_adjust,
        imaging::image_convert,
        imaging::image_apply_ops,
        video::video_trim,
        video::video_extract_frame,
        video::video_extract_audio,
        video::video_convert,
        video::video_convert_cancel,
        stems::stems_status,
        stems::stems_separate,
        stems::stems_install,
        stems::stems_cancel,
        convert::convert_capabilities,
        convert::convert_targets,
        convert::convert_file,
        convert::images_to_pdf,
        actions::merge_csv,
        actions::organize_dir,
        git::git_repo_root,
        git::git_status,
        git::git_current_branch,
        git::git_ahead_behind,
        git::git_branches,
        git::git_log,
        git::git_stage,
        git::git_unstage,
        git::git_commit,
        git::git_checkout_branch,
        git::git_diff_file,
        disk::disk_free,
        disk::path_exists,
        integrity::file_hash,
        integrity::file_kind,
        index::index_refresh,
        index::global_search,
        ocr::ocr_capabilities,
        ocr::ocr_extract,
    ]
}

/// Prépare Vela dans un hôte Tauri tiers : états gérés et index de recherche.
/// Ne lance ni le canal de contrôle ni FileManager1 (ils appartiennent à l'instance autonome).
/// Les plugins Tauri que le front Vela utilise (opener) doivent être enregistrés par l'hôte.
pub fn preparer_integre(app: &tauri::App) {
    use tauri::Manager;
    // Doit rester aligné sur les `.manage(...)` de `run()`.
    app.manage(PendingPath(std::sync::Mutex::new(None)));
    app.manage(picker::PickerState(std::sync::Mutex::new(None)));
    app.manage(archive::ExtractionManager::new());
    app.manage(watcher::DirWatcher::new());
    app.manage(ops::TransferManager::new());
    app.manage(terminal::TerminalManager::new());
    app.manage(player::PlayerManager::new());
    app.manage(video::VideoJobManager::new());
    app.manage(stems::StemsManager::new());
    app.manage(download_job::DownloadManager::new());
    app.manage(index::SearchIndex::new());
    app.manage(share::ShareManager::new());
    player::init();
    let idx = app.state::<index::SearchIndex>().inner().clone();
    std::thread::spawn(move || idx.rebuild_from_home());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entrees_du_gestionnaire() -> Vec<String> {
        let source = include_str!("lib.rs");
        let debut = source.find("tauri::generate_handler![").expect("gestionnaire introuvable");
        let corps = &source[debut..];
        let fin = corps.find("\n    ]").expect("fin du gestionnaire introuvable");
        corps[..fin]
            .lines()
            .skip(1)
            .map(|l| l.trim().trim_end_matches(',').to_string())
            .filter(|l| !l.is_empty())
            .collect()
    }

    #[test]
    fn la_liste_des_noms_couvre_le_gestionnaire() {
        let entrees = entrees_du_gestionnaire();
        assert_eq!(entrees.len(), NOMS_COMMANDES.len());
        for entree in &entrees {
            let nom = entree.rsplit("::").next().unwrap_or_default();
            assert!(est_commande_vela(nom), "commande absente de la liste : {nom}");
        }
    }

    #[test]
    fn pas_de_nom_en_double() {
        let mut noms = NOMS_COMMANDES.to_vec();
        noms.sort_unstable();
        noms.dedup();
        assert_eq!(noms.len(), NOMS_COMMANDES.len());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Dossier reçu en argument (clic sur un dossier, « ouvrir avec Vela »). Si une instance tourne
    // déjà, on l'y fait naviguer via le control plane puis on quitte — pas de seconde instance.
    let pick = picker::from_args();
    let picking = pick.is_some();
    let initial_dir = std::env::args().nth(1).filter(|a| std::path::Path::new(a).is_dir());
    if let (false, Some(dir)) = (picking, &initial_dir) {
        if control::forward("navigate", dir) {
            return;
        }
    }
    player::init();
    tauri::Builder::default()
        .manage(PendingPath(std::sync::Mutex::new(initial_dir)))
        .manage(picker::PickerState(std::sync::Mutex::new(pick)))
        .manage(archive::ExtractionManager::new())
        .manage(watcher::DirWatcher::new())
        .manage(ops::TransferManager::new())
        .manage(terminal::TerminalManager::new())
        .manage(player::PlayerManager::new())
        .manage(video::VideoJobManager::new())
        .manage(stems::StemsManager::new())
        .manage(download_job::DownloadManager::new())
        .manage(index::SearchIndex::new())
        .manage(share::ShareManager::new())
        .setup(move |app| {
            use tauri::Manager;
            // sélecteur de fichiers : fenêtre éphémère, sans index ni canal de contrôle (ceux de l'instance principale)
            if picking {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.set_title("Vela · sélection");
                }
                return Ok(());
            }
            let idx = app.state::<index::SearchIndex>().inner().clone();
            std::thread::spawn(move || idx.rebuild_from_home());
            match control::init(&app.handle().clone()) {
                Ok(plane) => { app.manage(plane); }
                Err(e) => eprintln!("[control] init impossible : {e}"),
            }
            filemanager1::init(&app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(gestionnaire_commandes())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

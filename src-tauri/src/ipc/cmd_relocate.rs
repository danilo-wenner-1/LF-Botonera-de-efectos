/// Modulo: cmd_relocate.rs
/// Proposito: reencontrar los audios que ya no estan donde decia la botonera
/// (borrados, movidos o la app instalada en otra PC). El usuario elige una
/// carpeta; Rust la recorre, empareja por nombre (ver `domain/button/relocate.rs`)
/// y reescribe la ruta en TODAS las superficies: botoneras, panel fijo y cola del
/// reproductor.
use super::AppState;
use crate::domain::button::relocate;
use crate::domain::library::path_rewrite::replace_in_config;
use crate::engine::audio::formats::audio_files_recursive;
use crate::engine::persist::config_io as config;
use crate::model::{AppConfig, ButtonData};
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Serialize)]
pub struct RelocateResult {
    /// Botones cuya ruta se corrigio.
    pub relocated: usize,
    /// Archivos distintos que siguen sin aparecer en la carpeta.
    pub still_missing: usize,
    /// Archivos distintos que faltaban antes de buscar.
    pub total_missing: usize,
}

/// Rutas de audio que no existen, sin repetir (un archivo usado en varios botones
/// cuenta una vez).
pub fn missing_paths(cfg: &AppConfig) -> Vec<String> {
    let mut found = BTreeSet::new();
    let mut take = |buttons: &[ButtonData]| {
        for button in buttons {
            if button.type_field == "audio" && relocate::is_missing(&button.path) {
                found.insert(button.path.clone());
            }
        }
    };
    take(&cfg.fixed_panel.global_buttons);
    take(&cfg.player.tracks);
    for profile in &cfg.profiles {
        take(&profile.fixed_buttons);
        for paleta in &profile.paletas {
            take(&paleta.botones);
        }
    }
    found.into_iter().collect()
}

/// Busca en `folder` (y subcarpetas) los audios que faltan y corrige sus rutas.
///
/// Asincrono: recorrer una carpeta grande tarda y un comando sincrono correria en
/// el hilo de la ventana. El recorrido va ANTES de tomar el candado de la
/// configuracion: no se retiene mientras se lee el disco.
#[tauri::command]
pub async fn relocate_missing_files(
    folder: String,
    state: tauri::State<'_, AppState>,
) -> Result<RelocateResult, String> {
    if !Path::new(&folder).is_dir() {
        return Err("folder_not_found".into());
    }
    let missing = missing_paths(&state.config.lock().unwrap());
    let total_missing = missing.len();
    if missing.is_empty() {
        return Ok(RelocateResult {
            relocated: 0,
            still_missing: 0,
            total_missing,
        });
    }
    let candidates = audio_files_recursive(&folder);
    let plan = relocate::plan(&missing, &candidates);
    let mut relocated = 0;
    if !plan.is_empty() {
        let mut cfg = state.config.lock().unwrap();
        state.history.lock().unwrap().remember(&cfg);
        for (old, new) in &plan {
            relocated += replace_in_config(&mut cfg, old, new);
        }
        config::save_config(&cfg)?;
    }
    Ok(RelocateResult {
        relocated,
        still_missing: total_missing - plan.len(),
        total_missing,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::button::defaults::new_button;

    #[test]
    fn solo_cuenta_los_audios_que_faltan_y_sin_repetir() {
        let mut cfg = AppConfig::default();
        let paleta = &mut cfg.profiles[0].paletas[0];
        for (i, path) in ["Z:/no/existe/a.mp3", "Z:/no/existe/a.mp3", ""].iter().enumerate() {
            let mut b = new_button("p", i as u32 + 1, "X", "#000000", "#FFFFFF");
            b.path = path.to_string();
            paleta.botones.push(b);
        }
        assert_eq!(missing_paths(&cfg), vec!["Z:/no/existe/a.mp3".to_string()]);
    }
}

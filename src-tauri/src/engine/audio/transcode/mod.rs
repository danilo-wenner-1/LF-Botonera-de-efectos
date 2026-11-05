//! Modulo: engine/audio/transcode/
//! Proposito: soporte transparente de M4A/MP4/AAC.
//!
//! Lo que symphonia reproduce con calidad completa (AAC-LC, ALAC) se reproduce
//! directo. Lo demas (HE-AAC, AAC Main/LTP, archivos que no abre) se convierte
//! UNA vez a MP3 320 kbps con el FFmpeg incluido, en segundo plano, y desde
//! entonces el decodificador lee la copia. Ver `probe.rs` y `cache.rs`.
//!
//! Reglas de estabilidad:
//! - Convertir NUNCA ocurre en el hilo de audio ni en el de la ventana: hay un
//!   unico trabajador con cola (importar 200 archivos no lanza 200 FFmpeg).
//! - Mientras la copia no esta lista, ese archivo no se decodifica a medias
//!   (sonaria opaco): el disparo devuelve `audio_converting` y la UI lo dice.
//! - Si la conversion falla se informa (`audio-convert-status`) y no se
//!   reintenta en bucle; reasignar el archivo lo vuelve a intentar.
pub mod cache;
pub mod ffmpeg;
pub mod probe;

use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Mutex, OnceLock};
use tauri::Emitter;

#[derive(Clone, PartialEq, Debug)]
enum Status {
    Native,
    Converting,
    Ready(PathBuf),
    Failed(String),
}

#[derive(Serialize, Clone)]
struct StatusEvent {
    path: String,
    status: &'static str,
    error: Option<String>,
}

struct Shared {
    /// Clave = copia destino (ruta + sello): un original modificado es otra clave.
    status: Mutex<HashMap<PathBuf, Status>>,
    queue: Sender<(String, PathBuf)>,
}

static SHARED: OnceLock<Shared> = OnceLock::new();
static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

/// Para avisar a la UI del resultado de cada conversion. Se fija al arrancar.
pub fn set_app(app: tauri::AppHandle) {
    let _ = APP.set(app);
}

fn shared() -> &'static Shared {
    SHARED.get_or_init(|| {
        let (queue, rx) = channel::<(String, PathBuf)>();
        let _ = std::thread::Builder::new()
            .name("audio-transcode".into())
            .spawn(move || {
                for (original, target) in rx {
                    let result = ffmpeg::to_mp3(&original, &target);
                    finish(&original, &target, result);
                }
            });
        Shared {
            status: Mutex::new(HashMap::new()),
            queue,
        }
    })
}

fn finish(original: &str, target: &Path, result: Result<(), String>) {
    let status = match result {
        Ok(()) => {
            cache::remove_stale(original, target);
            Status::Ready(target.to_path_buf())
        }
        Err(error) => Status::Failed(error),
    };
    let event = StatusEvent {
        path: original.to_string(),
        status: if matches!(status, Status::Ready(_)) { "ready" } else { "failed" },
        error: match &status {
            Status::Failed(e) => Some(e.clone()),
            _ => None,
        },
    };
    shared().status.lock().unwrap().insert(target.to_path_buf(), status);
    if let Some(app) = APP.get() {
        let _ = app.emit("audio-convert-status", event);
    }
}

/// Decide (y recuerda) que hacer con un archivo; encola la conversion si falta.
fn status_of(original: &str) -> Option<Status> {
    if !probe::is_candidate(original) {
        return None;
    }
    let target = cache::target_for(original)?;
    let shared = shared();
    if let Some(known) = shared.status.lock().unwrap().get(&target) {
        return Some(known.clone());
    }
    let status = if target.is_file() {
        Status::Ready(target.clone())
    } else if probe::plays_natively(original) {
        Status::Native
    } else if ffmpeg::binary().is_none() {
        Status::Failed("audio_converter_missing".into())
    } else {
        let _ = shared.queue.send((original.to_string(), target.clone()));
        Status::Converting
    };
    shared.status.lock().unwrap().insert(target, status.clone());
    Some(status)
}

/// La ruta que debe leer el decodificador. None = todavia no se puede leer con
/// calidad completa (se esta convirtiendo, o fallo y no se reproduce directo).
pub fn resolve(original: &str) -> Option<String> {
    match status_of(original) {
        None | Some(Status::Native) => Some(original.to_string()),
        Some(Status::Ready(copy)) => Some(copy.to_string_lossy().to_string()),
        Some(Status::Converting) => None,
        // Sin conversor, mejor que suene directo (aunque sea peor) que nada.
        Some(Status::Failed(e)) if e == "audio_converter_missing" => {
            Some(original.to_string())
        }
        Some(Status::Failed(_)) => None,
    }
}

/// Lo que impide disparar ahora un archivo, como codigo de error para la UI.
pub fn play_blocker(original: &str) -> Option<String> {
    match status_of(original)? {
        Status::Converting => Some("audio_converting".into()),
        Status::Failed(e) if e != "audio_converter_missing" => Some(e),
        _ => None,
    }
}

/// Al importar/asignar: prepara el archivo (encola la conversion si hace
/// falta) y olvida un fallo anterior para reintentarlo.
pub fn prepare(original: &str) -> Result<(), String> {
    if let Some(target) = cache::target_for(original) {
        let mut map = shared().status.lock().unwrap();
        if matches!(map.get(&target), Some(Status::Failed(_))) {
            map.remove(&target);
        }
    }
    match status_of(original) {
        Some(Status::Failed(e)) if e == "audio_converter_missing" => {
            // Sin conversor se acepta lo que al menos se pueda decodificar.
            if crate::engine::audio::decode::can_decode(original) {
                Ok(())
            } else {
                Err("unsupported_audio_format".into())
            }
        }
        _ => Ok(()),
    }
}

//! Modulo: engine/audio/transcode/cache.rs
//! Proposito: donde vive la copia convertida de cada archivo y como se llama.
//!
//! `%APPDATA%/LF Botonera Radio/converted/{ruta}-{sello}.mp3`:
//! - `ruta`  = huella de la ruta original: el mismo archivo siempre cae en el
//!   mismo sitio y no se duplica;
//! - `sello` = huella de fecha de modificacion + tamaño: si el original cambia,
//!   la copia vieja deja de valer y se sustituye (no se acumulan versiones).
//!
//! El ORIGINAL nunca se toca: la configuracion, la Biblioteca y `tracks.db`
//! siguen apuntando a el; solo el decodificador lee la copia.
use crate::engine::audio::formats::file_stamp;
use crate::engine::persist::config_io::get_data_dir;
use crate::engine::persist::db::normalize_key;
use std::path::{Path, PathBuf};

pub fn dir() -> PathBuf {
    get_data_dir().join("converted")
}

/// FNV-1a de 64 bits: estable entre versiones de Rust (`DefaultHasher` no lo
/// promete) y de sobra para nombrar archivos.
pub fn fnv(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
        (hash ^ *byte as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// Prefijo comun a todas las versiones convertidas de una misma ruta.
pub fn path_prefix(original: &str) -> String {
    format!("{:016x}-", fnv(normalize_key(original).as_bytes()))
}

/// Nombre de la copia para el estado ACTUAL del original. None si no existe.
pub fn target_for(original: &str) -> Option<PathBuf> {
    let (mtime, size) = file_stamp(Path::new(original));
    if size == 0 {
        return None;
    }
    let stamp = fnv(format!("{mtime}:{size}").as_bytes());
    Some(dir().join(format!("{}{stamp:016x}.mp3", path_prefix(original))))
}

/// Borra las copias de versiones anteriores del mismo original.
pub fn remove_stale(original: &str, keep: &Path) {
    let prefix = path_prefix(original);
    let Ok(entries) = std::fs::read_dir(dir()) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(&prefix) && path != keep {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_huella_es_estable() {
        // Valor de referencia de FNV-1a 64 para "a": si cambiara, todas las
        // copias ya convertidas se volverian a convertir.
        assert_eq!(fnv(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn la_misma_ruta_comparte_prefijo() {
        assert_eq!(path_prefix("C:/fx/a.m4a"), path_prefix("C:/fx/a.m4a"));
        assert_ne!(path_prefix("C:/fx/a.m4a"), path_prefix("C:/fx/b.m4a"));
    }
}

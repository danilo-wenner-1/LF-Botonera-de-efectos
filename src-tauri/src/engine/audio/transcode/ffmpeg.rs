//! Modulo: engine/audio/transcode/ffmpeg.rs
//! Proposito: localizar el FFmpeg incluido con la aplicacion y convertir.
//!
//! Se distribuye como binario externo de Tauri (`bundle.externalBin`), que lo
//! deja junto al ejecutable como `ffmpeg(.exe)` tanto en el instalador como en
//! `tauri dev`. Si no esta ahi (Linux con el de su distribucion), se usa el del
//! PATH. Se ejecuta como proceso aparte: un archivo dañado puede tumbar a
//! FFmpeg, nunca a la botonera.
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

const EXE: &str = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };

/// La ruta del FFmpeg utilizable, comprobada una vez por sesion.
pub fn binary() -> Option<&'static Path> {
    static FOUND: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOUND
        .get_or_init(|| {
            let beside = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|dir| dir.join(EXE)));
            beside
                .into_iter()
                .chain(std::iter::once(PathBuf::from(EXE)))
                .find(|candidate| works(candidate))
        })
        .as_deref()
}

fn works(candidate: &Path) -> bool {
    command(candidate)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Convierte `input` a MP3 320 kbps en `output`. Escribe primero a un archivo
/// temporal y lo renombra al acabar: una conversion cortada (apagon, cierre)
/// nunca deja una copia a medias que pareceria valida.
pub fn to_mp3(input: &str, output: &Path) -> Result<(), String> {
    let ffmpeg = binary().ok_or("audio_converter_missing")?;
    if let Some(dir) = output.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let partial = output.with_extension("part");
    let result = command(ffmpeg)
        .args(["-nostdin", "-hide_banner", "-loglevel", "error", "-y", "-i"])
        .arg(input)
        .args(["-map", "0:a:0", "-vn", "-c:a", "libmp3lame", "-b:a", "320k"])
        .args(["-id3v2_version", "3", "-f", "mp3"])
        .arg(&partial)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        let _ = std::fs::remove_file(&partial);
        let detail = String::from_utf8_lossy(&result.stderr);
        eprintln!("ffmpeg no pudo convertir {input}: {}", detail.trim());
        return Err("audio_convert_failed".into());
    }
    std::fs::rename(&partial, output).map_err(|e| {
        let _ = std::fs::remove_file(&partial);
        e.to_string()
    })
}

/// En Windows, sin ventana de consola parpadeando en pleno directo.
fn command(program: &Path) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

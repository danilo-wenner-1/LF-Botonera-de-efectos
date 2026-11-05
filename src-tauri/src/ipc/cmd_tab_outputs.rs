/// Modulo: cmd_tab_outputs.rs
/// Proposito: salida de audio independiente por pestaña. Lee la configuracion,
/// pregunta que tarjetas hay conectadas y aplica el plan de carriles
/// (`domain/console/tab_lanes.rs`). La UI solo elige la tarjeta y muestra avisos.
use super::AppState;
use crate::domain::console::tab_lanes::{self, OutputIssue, TabRoute};
use crate::engine::console::device as audio_device;
use crate::engine::console::BusId;
use crate::engine::persist::config_io as config;
use serde::Serialize;

#[derive(Serialize)]
pub struct TabOutputStatus {
    /// Tarjetas visibles ahora (para el selector).
    pub devices: Vec<String>,
    /// Pestañas que no pueden salir por la tarjeta que eligieron. Suenan por la
    /// salida principal hasta que se corrija.
    pub issues: Vec<OutputIssue>,
}

/// Recalcula y aplica los carriles del perfil activo. Se llama al arrancar y
/// cada vez que cambia algo que los afecta (salida de una pestaña, salida
/// principal, perfil activo, pestañas creadas, borradas o importadas).
pub fn apply(state: &AppState) -> TabOutputStatus {
    let devices = audio_device::available_devices();
    // Antes de tomar la configuracion: nunca se sostienen los dos candados.
    let previous = state.audio.lock().unwrap().lane_devices();
    let plan = {
        let cfg = state.config.lock().unwrap();
        let Some(profile) = cfg.active_profile() else {
            return TabOutputStatus {
                devices,
                issues: Vec::new(),
            };
        };
        let tabs: Vec<TabRoute> = profile
            .paletas
            .iter()
            .map(|p| TabRoute {
                paleta_id: &p.id,
                nombre: &p.nombre,
                audio_out: &p.audio_out,
            })
            .collect();
        let mut plan = tab_lanes::plan(&tabs, &profile.audio.out_main, &devices, &previous);
        // Un carril que ya se intento abrir (mismo carril, misma tarjeta) y no
        // esta vivo: la tarjeta figura pero no abre (otro programa la tiene en
        // exclusivo). Sus botones suenan por la principal; hay que decirlo.
        for tab in &tabs {
            let device = tab.audio_out.trim();
            let Some(lane) = plan.lanes.get(device) else {
                continue;
            };
            if previous.get(device) == Some(lane)
                && state.console.bus(BusId::Salida(*lane)).is_none()
            {
                plan.issues.push(OutputIssue {
                    paleta_id: tab.paleta_id.to_string(),
                    nombre: tab.nombre.to_string(),
                    device: device.to_string(),
                    reason: "unavailable",
                });
            }
        }
        plan
    };
    let issues = plan.issues.clone();
    let _ = state.audio.lock().unwrap().set_tab_lanes(plan);
    TabOutputStatus { devices, issues }
}

/// Estado de las salidas por pestaña (y las reaplica: una tarjeta reconectada
/// vuelve a recibir su carril, y una desconectada lo suelta para que sus
/// pestañas suenen por la principal). La UI lo consulta cada pocos segundos
/// mientras alguna pestaña tenga salida propia.
///
/// Asincrono a proposito: listar tarjetas cuesta milisegundos y un comando
/// sincrono correria en el hilo de la ventana. Reaplicar el mismo plan no
/// rehace el grafo (lo comprueba la consola), asi que no corta nada.
#[tauri::command]
pub async fn get_tab_output_status(
    state: tauri::State<'_, AppState>,
) -> Result<TabOutputStatus, String> {
    Ok(apply(&state))
}

/// Cambia solo la salida de una pestaña. "" = la salida principal.
#[tauri::command]
pub fn set_tab_output(
    profile_id: String,
    paleta_id: String,
    device: String,
    state: tauri::State<AppState>,
) -> Result<TabOutputStatus, String> {
    {
        let mut cfg = state.config.lock().unwrap();
        let paleta = cfg
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .and_then(|p| p.paletas.iter_mut().find(|t| t.id == paleta_id))
            .ok_or("Pestaña no encontrada")?;
        paleta.audio_out = device.trim().to_string();
        config::save_config(&cfg)?;
    }
    Ok(apply(&state))
}

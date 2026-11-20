//! Los atajos de boton suenan desde CUALQUIER pestana del perfil, y por eso el
//! duplicado se detecta en todas ellas.
use super::actions::button_with_shortcut;
use super::rules::apply_button_shortcut;
use crate::domain::button::defaults::new_button;
use crate::model::{AppConfig, PaletaData};

fn tab(id: &str) -> PaletaData {
    PaletaData {
        id: id.into(),
        nombre: format!("Pestaña {id}"),
        rows: 5,
        cols: 5,
        audio_out: String::new(),
        shortcut: String::new(),
        midi: Default::default(),
        tab_bg: String::new(),
        tab_text: String::new(),
        font_size: 0,
        botones: Vec::new(),
    }
}

/// Perfil con dos pestañas: la activa y otra con un boton que tiene atajo.
fn config_with_shortcut_in_other_tab(key: &str) -> AppConfig {
    let mut cfg = AppConfig::default();
    let profile = &mut cfg.profiles[0];
    let active = profile.active_paleta_id.clone();
    profile.paletas.clear();
    let mut first = tab(&active);
    let mut second = tab("otra");
    let mut button = new_button("otra", 3, "Aplausos", "#000000", "#FFFFFF");
    button.shortcut = key.into();
    second.botones.push(button);
    first.botones.push(new_button(&active, 1, "Lluvia", "#000000", "#FFFFFF"));
    profile.paletas.push(first);
    profile.paletas.push(second);
    cfg
}

#[test]
fn un_atajo_de_otra_pestana_encuentra_su_boton() {
    let cfg = config_with_shortcut_in_other_tab("F5");
    let found = button_with_shortcut(&cfg.profiles[0], "F5");
    assert_eq!(found.as_deref(), Some("otra_btn_3"));
    assert_eq!(button_with_shortcut(&cfg.profiles[0], "F6"), None);
}

/// Una configuracion antigua pudo repetir el atajo en dos pestañas (antes solo
/// sonaba la visible): gana la de la pestaña que se esta viendo.
#[test]
fn con_un_duplicado_antiguo_gana_la_pestana_visible() {
    let mut cfg = config_with_shortcut_in_other_tab("F5");
    let active = cfg.profiles[0].active_paleta_id.clone();
    cfg.profiles[0].paletas[0].botones[0].shortcut = "F5".into();
    let found = button_with_shortcut(&cfg.profiles[0], "F5").unwrap();
    assert_eq!(found, format!("{active}_btn_1"));
}

#[test]
fn asignar_un_atajo_ya_usado_en_otra_pestana_avisa() {
    let mut cfg = config_with_shortcut_in_other_tab("F5");
    let err = apply_button_shortcut(&mut cfg, 1, "F5", false).unwrap_err();
    assert!(err.starts_with("shortcut_conflict_button_any|F5|"), "{err}");
    assert!(err.contains("Aplausos"), "dice cual es: {err}");
}

#[test]
fn reemplazar_quita_el_atajo_del_otro_boton() {
    let mut cfg = config_with_shortcut_in_other_tab("F5");
    apply_button_shortcut(&mut cfg, 1, "F5", true).unwrap();
    assert!(cfg.profiles[0].paletas[1].botones[0].shortcut.is_empty());
}

/// Reasignar el atajo que el propio boton ya tiene no es un conflicto.
#[test]
fn el_propio_boton_no_choca_consigo_mismo() {
    let mut cfg = config_with_shortcut_in_other_tab("F5");
    cfg.profiles[0].active_paleta_id = "otra".into();
    assert!(apply_button_shortcut(&mut cfg, 3, "F5", false).is_ok());
}

#[test]
fn la_visualizacion_nace_como_siempre() {
    let cfg: AppConfig = serde_json::from_str("{}").unwrap();
    assert!(cfg.show_button_index);
    assert_eq!(cfg.duration_font_px, 13);
}

//! Compatibilidad de los campos nuevos de presentacion (estilo de boton y
//! tamaño de pestaña) y de la barra inferior: un archivo antiguo carga igual y
//! un archivo nuevo sin personalizar no cambia de forma.
use crate::domain::button::defaults::new_button;
use crate::domain::export::lfa_format::{from_lfa_paleta, to_lfa_paleta};
use crate::model::{AppConfig, ButtonData, ButtonStyle, PaletaData};

const OLD_BUTTON: &str = r##"{"id":"p_btn_1","index":1,"label":"Aplausos",
    "path":"C:/fx/a.mp3","color_bg":"#E53935","color_text":"#FFFFFF"}"##;

#[test]
fn un_boton_antiguo_carga_sin_estilo_y_se_guarda_igual() {
    let button: ButtonData = serde_json::from_str(OLD_BUTTON).unwrap();
    assert!(button.style.is_default());
    let json = serde_json::to_string(&button).unwrap();
    assert!(!json.contains("\"style\""), "no debe aparecer mientras no se use");
}

#[test]
fn el_estilo_propio_se_guarda_y_se_recupera() {
    let mut button: ButtonData = serde_json::from_str(OLD_BUTTON).unwrap();
    button.style = ButtonStyle {
        font_size: 18,
        gradient: "diagonal".into(),
        bg2: "#000000".into(),
        text2: "#FFD700".into(),
        exact: true,
    };
    let back: ButtonData =
        serde_json::from_str(&serde_json::to_string(&button).unwrap()).unwrap();
    assert_eq!(back.style, button.style);
}

#[test]
fn la_pestana_omite_su_tamano_si_es_el_global() {
    let paleta: PaletaData = serde_json::from_str(r#"{"id":"p","nombre":"FX","botones":[]}"#).unwrap();
    assert_eq!(paleta.font_size, 0);
    assert!(!serde_json::to_string(&paleta).unwrap().contains("font_size"));
}

#[test]
fn la_barra_inferior_ausente_significa_visible() {
    let cfg: AppConfig = serde_json::from_str("{}").unwrap();
    assert!(cfg.show_bottom_bar);
    assert!(AppConfig::default().show_bottom_bar);
}

/// Exportar e importar una pestaña (.bdelf) conserva tamaño, degradado y color
/// exacto; el LF Automatizador ignora esos campos.
#[test]
fn el_estilo_sobrevive_al_formato_compartido() {
    let mut button = new_button("src", 1, "Cortina", "#112233", "#FFFFFF");
    button.path = "cortina.wav".into();
    button.style = ButtonStyle {
        font_size: 22,
        gradient: "vertical".into(),
        bg2: "#445566".into(),
        text2: String::new(),
        exact: true,
    };
    let source = PaletaData {
        id: "src".into(),
        nombre: "Cortinas".into(),
        rows: 1,
        cols: 1,
        audio_out: "Auriculares".into(),
        shortcut: String::new(),
        midi: Default::default(),
        tab_bg: String::new(),
        tab_text: String::new(),
        font_size: 18,
        botones: vec![button],
    };
    let json = serde_json::to_string(&to_lfa_paleta(&source)).unwrap();
    let restored = from_lfa_paleta(serde_json::from_str(&json).unwrap(), "dst".into());
    assert_eq!(restored.font_size, 18);
    assert_eq!(restored.audio_out, "Auriculares");
    assert_eq!(restored.botones[0].style, source.botones[0].style);
}

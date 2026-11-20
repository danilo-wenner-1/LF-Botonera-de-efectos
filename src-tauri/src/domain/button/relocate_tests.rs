//! Pruebas de la busqueda de audios perdidos.
use super::{best_match, is_missing, plan};

fn list(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn encuentra_por_nombre_sin_importar_mayusculas_ni_separadores() {
    let found = list(&["D:/Audios/FX/APLAUSOS.MP3"]);
    assert_eq!(
        best_match(r"C:\Users\otro\fx\aplausos.mp3", &found),
        Some(&found[0])
    );
}

#[test]
fn si_el_nombre_no_esta_no_hay_resultado() {
    assert_eq!(best_match("C:/fx/lluvia.mp3", &list(&["D:/a/otra.mp3"])), None);
    assert_eq!(best_match("", &list(&["D:/a/otra.mp3"])), None);
}

/// Dos archivos con el mismo nombre: gana el que comparte mas carpetas finales.
#[test]
fn con_nombres_repetidos_gana_la_ruta_mas_parecida() {
    let found = list(&["E:/Cortinas/a1.mp3", "E:/Efectos/Aplausos/a1.mp3"]);
    let best = best_match("C:/Radio/Efectos/Aplausos/a1.mp3", &found).unwrap();
    assert_eq!(best, "E:/Efectos/Aplausos/a1.mp3");
}

/// El orden en que el disco enumera no cambia el resultado.
#[test]
fn el_resultado_no_depende_del_orden() {
    let a = list(&["E:/x/z.mp3", "E:/y/z.mp3"]);
    let b = list(&["E:/y/z.mp3", "E:/x/z.mp3"]);
    assert_eq!(best_match("C:/q/z.mp3", &a), best_match("C:/q/z.mp3", &b));
}

#[test]
fn el_plan_solo_incluye_lo_que_se_encontro() {
    let missing = list(&["C:/fx/a.mp3", "C:/fx/b.wav", "C:/fx/c.m4a"]);
    let folder = list(&["F:/nuevo/A.mp3", "F:/nuevo/sub/c.m4a"]);
    let plan = plan(&missing, &folder);
    assert_eq!(plan.len(), 2);
    assert_eq!(plan["C:/fx/a.mp3"], "F:/nuevo/A.mp3");
    assert!(!plan.contains_key("C:/fx/b.wav"));
}

#[test]
fn una_ruta_vacia_no_cuenta_como_perdida() {
    assert!(!is_missing(""));
    assert!(!is_missing("   "));
    assert!(is_missing("Z:/esto/no/existe/nunca.mp3"));
}

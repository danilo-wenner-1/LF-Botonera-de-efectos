/// Modulo: engine/audio/routing.rs
/// Proposito: a que bus de la consola manda el motor de efectos cada fuente.
/// Es todo lo que queda del booleano `to_pre`, que elegia entre dos salidas
/// fijas. Ahora la pre-escucha tiene bus propio y los botones van al de su
/// grupo, asi que cada uno puede tener su fader y su vumetro.
use crate::engine::audio::button::PlaybackGroup;
use crate::engine::console::{Bus, BusId, ConsoleEngine};

pub fn bus_for(to_pre: bool, group: PlaybackGroup) -> BusId {
    if to_pre {
        return BusId::Cue;
    }
    match group {
        PlaybackGroup::Main => BusId::Efectos,
        PlaybackGroup::Fixed => BusId::Panel,
        PlaybackGroup::Cue => BusId::Cue,
    }
}

/// pestaña → carril. Solo estan las pestañas con tarjeta propia conectada.
pub type TabBuses = std::collections::HashMap<String, BusId>;

/// El bus que pide un boton: el carril de su pestaña si lo tiene; si no, el de
/// su grupo. La pre-escucha y el panel fijo nunca usan carriles de pestaña.
pub fn bus_for_button(tabs: &TabBuses, id: &str, to_pre: bool, group: PlaybackGroup) -> BusId {
    if !to_pre && group == PlaybackGroup::Main {
        if let Some(bus) = crate::domain::console::tab_lanes::paleta_of_button(id)
            .and_then(|paleta| tabs.get(paleta))
        {
            return *bus;
        }
    }
    bus_for(to_pre, group)
}

/// El bus vivo por el que sonara una fuente que pidio `bus`.
///
/// Un carril de pestaña puede no existir (su tarjeta se desconecto o no abrio).
/// Entonces el boton suena por la principal (Efectos): en una radio, un efecto
/// que sale por el sitio equivocado se corrige; uno que no sale no. Los buses
/// fijos no tienen alternativa — la pre-escucha jamas debe caer al aire.
pub fn resolve(console: &ConsoleEngine, bus: BusId) -> Option<(BusId, Bus)> {
    if let Some(live) = console.bus(bus) {
        return Some((bus, live));
    }
    match bus {
        BusId::Salida(_) => console.bus(BusId::Efectos).map(|b| (BusId::Efectos, b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::bus_for;
    use crate::engine::audio::button::PlaybackGroup;
    use crate::engine::console::BusId;

    /// La pre-escucha va a su bus **sea cual sea el grupo**: que la dispare un
    /// boton del panel no la convierte en programa.
    #[test]
    fn la_pre_escucha_siempre_va_al_cue() {
        for group in [PlaybackGroup::Main, PlaybackGroup::Fixed] {
            assert_eq!(bus_for(true, group), BusId::Cue);
        }
    }

    #[test]
    fn cada_grupo_va_a_su_bus() {
        assert_eq!(bus_for(false, PlaybackGroup::Main), BusId::Efectos);
        assert_eq!(bus_for(false, PlaybackGroup::Fixed), BusId::Panel);
        assert_eq!(bus_for(false, PlaybackGroup::Cue), BusId::Cue);
    }

    /// Un boton de una pestaña con salida propia va a su carril; el panel fijo,
    /// la pre-escucha y las pestañas sin carril, a su bus de siempre.
    #[test]
    fn cada_boton_va_al_carril_de_su_pestana() {
        use super::{bus_for_button, TabBuses};
        let mut tabs = TabBuses::new();
        tabs.insert("perfil_paleta_2".into(), BusId::Salida(3));
        let main = PlaybackGroup::Main;
        assert_eq!(bus_for_button(&tabs, "perfil_paleta_2_btn_4", false, main), BusId::Salida(3));
        assert_eq!(bus_for_button(&tabs, "perfil_paleta_1_btn_4", false, main), BusId::Efectos);
        assert_eq!(bus_for_button(&tabs, "perfil_paleta_2_btn_4", true, main), BusId::Cue);
        assert_eq!(
            bus_for_button(&tabs, "perfil_paleta_2_btn_4", false, PlaybackGroup::Fixed),
            BusId::Panel
        );
    }

    /// Los dos buses de botones suman en programa: el vumetro principal cuenta
    /// la botonera Y el panel fijo, que es lo que el operador espera ver.
    #[test]
    fn los_dos_buses_de_botones_suenan_en_programa() {
        assert!(bus_for(false, PlaybackGroup::Main).can_sum_into_program());
        assert!(bus_for(false, PlaybackGroup::Fixed).can_sum_into_program());
        assert!(!bus_for(true, PlaybackGroup::Main).can_sum_into_program());
    }
}

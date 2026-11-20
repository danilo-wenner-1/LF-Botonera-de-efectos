/**
 * Archivo: displayOptions.js
 * Propósito: lo que se ve en cada botón de la botonera, como ajuste global: el
 * número de posición (esquina superior izquierda) y el tamaño del texto de
 * duración (bajo el nombre). Solo dibuja: Rust guarda y valida (set_button_display).
 */
import { invoke } from '../bridge/api.js';

const DEFAULT_DURATION_PX = 13;

/** Aplica la configuración a la página. `show_button_index` ausente = visible. */
export function applyDisplayOptions(config) {
    document.body.classList.toggle('hide-button-index', config?.show_button_index === false);
    const px = config?.duration_font_px || DEFAULT_DURATION_PX;
    document.body.style.setProperty('--timer-font-size', `${px}px`);
}

/** Pone los controles de Ajustes como está la configuración. */
export function loadDisplayOptions(config) {
    document.getElementById('config-show-button-index').checked =
        config.show_button_index ?? true;
    document.getElementById('config-duration-size').value =
        String(config.duration_font_px || DEFAULT_DURATION_PX);
}

/** Guarda y aplica en el acto. */
export async function saveDisplayOptions() {
    const showIndex = document.getElementById('config-show-button-index').checked;
    const durationPx = Number(document.getElementById('config-duration-size').value)
        || DEFAULT_DURATION_PX;
    await invoke('set_button_display', { showIndex, durationPx });
    applyDisplayOptions({ show_button_index: showIndex, duration_font_px: durationPx });
}

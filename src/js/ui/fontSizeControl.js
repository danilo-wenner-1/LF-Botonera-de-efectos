/**
 * Archivo: fontSizeControl.js
 * Propósito: control − / valor / + / ↺ del tamaño de texto. Lo comparten el
 * editor de botón (tamaño propio) y el de pestaña (predeterminado). 0 = heredar.
 */
import { t } from '../util/i18n.js';
import { FONT_MIN, FONT_MAX } from './buttonStyle.js';

/** Tamaño en px de cada opción global de Ajustes (grid.css). */
const GLOBAL_PX = { small: 12, normal: 14, large: 17, xlarge: 20 };

export function globalFontPx() {
    return GLOBAL_PX[document.body.dataset.buttonTextSize] ?? 14;
}

/**
 * @param {string}   prefix     Prefijo de los ids (`edit` → #edit-font-inc…).
 * @param {Function} inherited  Tamaño heredado cuando el valor es 0.
 * @param {string}   inheritKey Clave i18n que describe de dónde se hereda.
 * @param {Function} onChange   Se llama con el valor nuevo.
 */
export function bindFontControl(prefix, inherited, inheritKey, onChange = () => {}) {
    const out = document.getElementById(`${prefix}-font-value`);
    let value = 0;
    const paint = () => {
        out.textContent = value
            ? `${value} px`
            : `${t(typeof inheritKey === 'function' ? inheritKey() : inheritKey)} (${inherited()} px)`;
        onChange(value);
    };
    const step = delta => {
        const base = value || inherited();
        value = Math.min(FONT_MAX, Math.max(FONT_MIN, base + delta));
        paint();
    };
    document.getElementById(`${prefix}-font-inc`).onclick = () => step(1);
    document.getElementById(`${prefix}-font-dec`).onclick = () => step(-1);
    document.getElementById(`${prefix}-font-reset`).onclick = () => { value = 0; paint(); };
    return {
        get: () => value,
        set: v => { value = Number(v) || 0; paint(); },
    };
}

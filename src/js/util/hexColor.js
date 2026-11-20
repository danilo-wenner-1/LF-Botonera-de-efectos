/**
 * Archivo: hexColor.js
 * Propósito: interpretar lo que el usuario escribe o pega como color HEX.
 * Rust vuelve a validar el formato al guardar (model/button_style.rs).
 */

/** `ff0000`, `#F00`, ` #ff0000 ` → `#FF0000`. null si no es un color válido. */
export function normalizeHex(raw) {
    let v = String(raw ?? '').trim().replace(/^#/, '');
    if (/^[0-9a-f]{3}$/i.test(v)) v = v.split('').map(c => c + c).join('');
    return /^[0-9a-f]{6}$/i.test(v) ? `#${v.toUpperCase()}` : null;
}

/**
 * Archivo: colorPalette.js
 * Propósito: la elección de colores de TODA la app, con un único menú: el
 * selector de color del sistema más un campo HEX (escribir o pegar #RRGGBB) y
 * vista previa. Sustituye a la antigua paleta de 24 colores.
 *
 * Dos usos:
 *  - `attachPalette` añade el campo HEX junto a un `<input type="color">` ya
 *    existente (pestañas, perfiles).
 *  - `pickColor` abre un diálogo y devuelve el color elegido (pintar varios
 *    botones a la vez, donde no hay formulario).
 */

import { normalizeHex } from '../util/hexColor.js';
import { t } from '../util/i18n.js';

const _pairs = [];

/** Pone un campo HEX junto a cada selector de color y los mantiene sincronizados. */
export function attachPalette(bgInput, textInput) {
    [bgInput, textInput].forEach(input => input && _attachHex(input));
    return Promise.resolve();
}

/** Repinta los campos HEX tras cambiar los colores desde código. */
export function refreshColorInputs() {
    _pairs.forEach(({ color, hex }) => {
        hex.value = normalizeHex(color.value) ?? '';
        hex.classList.remove('invalid');
    });
    return Promise.resolve();
}

function _attachHex(color) {
    if (color.dataset.hexReady) return;
    color.dataset.hexReady = '1';
    color.classList.add('native-color');
    const hex = _hexInput();
    const wrap = document.createElement('div');
    wrap.className = 'color-hex-pair';
    color.replaceWith(wrap);
    wrap.append(color, hex);
    hex.value = normalizeHex(color.value) ?? '';
    color.addEventListener('input', () => {
        hex.value = normalizeHex(color.value) ?? '';
        hex.classList.remove('invalid');
    });
    hex.addEventListener('input', () => {
        const value = normalizeHex(hex.value);
        hex.classList.toggle('invalid', !value && hex.value.trim() !== '');
        if (!value) return;
        color.value = value.toLowerCase();
        // Quien escuche el selector (vistas previas) se entera igual que si lo
        // hubiera movido el usuario.
        color.dispatchEvent(new Event('input', { bubbles: true }));
    });
    _pairs.push({ color, hex });
}

function _hexInput() {
    const hex = document.createElement('input');
    hex.type = 'text';
    hex.className = 'hex-input';
    hex.maxLength = 7;
    hex.spellcheck = false;
    hex.autocomplete = 'off';
    hex.placeholder = '#RRGGBB';
    hex.title = t('colors.hex_hint');
    return hex;
}

// ── Diálogo "elige un color y devuélvemelo" ─────────────────────────────────

let _resolvePick = null;
const $ = id => document.getElementById(id);

/** Abre el diálogo y devuelve el color elegido (`#RRGGBB`), o null si se cancela. */
export function pickColor(_role = 'button', initial = '#E53935') {
    const start = normalizeHex(initial) ?? '#E53935';
    $('color-picker-native').value = start.toLowerCase();
    $('color-picker-hex').value = start;
    $('color-picker-hex').classList.remove('invalid');
    _paintPreview(start);
    $('color-picker-modal')?.classList.remove('hidden');
    $('color-picker-hex')?.select();
    $('color-picker-hex')?.focus();
    return new Promise(resolve => { _resolvePick = resolve; });
}

export function initColorPicker() {
    $('color-picker-apply')?.addEventListener('click', _apply);
    $('color-picker-cancel')?.addEventListener('click', () => _finish(null));
    $('color-picker-modal')?.addEventListener('click', e => {
        if (e.target?.id === 'color-picker-modal') _finish(null);
    });
    $('color-picker-native')?.addEventListener('input', () => {
        const value = normalizeHex($('color-picker-native').value);
        $('color-picker-hex').value = value ?? '';
        $('color-picker-hex').classList.remove('invalid');
        if (value) _paintPreview(value);
    });
    $('color-picker-hex')?.addEventListener('input', () => {
        const hex = $('color-picker-hex');
        const value = normalizeHex(hex.value);
        hex.classList.toggle('invalid', !value && hex.value.trim() !== '');
        if (!value) return;
        $('color-picker-native').value = value.toLowerCase();
        _paintPreview(value);
    });
    $('color-picker-hex')?.addEventListener('keydown', e => {
        if (e.key === 'Enter') { e.preventDefault(); _apply(); }
    });
}

function _apply() {
    const value = normalizeHex($('color-picker-hex').value);
    if (!value) {
        $('color-picker-hex').classList.add('invalid');
        return;
    }
    _finish(value);
}

function _finish(value) {
    $('color-picker-modal')?.classList.add('hidden');
    const resolve = _resolvePick;
    _resolvePick = null;
    resolve?.(value);
}

/** El texto de la vista previa se elige por contraste con el fondo. */
function _paintPreview(bg) {
    const preview = $('color-picker-preview');
    if (!preview) return;
    preview.style.backgroundColor = bg;
    preview.style.color = _readableOn(bg);
}

function _readableOn(hex) {
    const n = parseInt(hex.slice(1), 16);
    const lin = v => {
        v /= 255;
        return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
    };
    const lum = 0.2126 * lin((n >> 16) & 255) + 0.7152 * lin((n >> 8) & 255) + 0.0722 * lin(n & 255);
    // Blanco o casi negro: el que más contraste dé (igual que `readable_text` en Rust).
    return (lum + 0.05) / 0.05 > 1.05 / (lum + 0.05) ? '#111111' : '#FFFFFF';
}

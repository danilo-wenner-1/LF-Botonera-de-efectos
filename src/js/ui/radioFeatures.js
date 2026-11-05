/**
 * Archivo: radioFeatures.js
 * Propósito: arranque de las funciones de operación en directo, en un único
 * sitio para no engordar startup.js: buscador global, rueda del ratón sobre las
 * pestañas, barra inferior ocultable, avisos de salidas por pestaña y avisos de
 * conversión de M4A.
 */
import { invoke, listen } from '../bridge/api.js';
import { t } from '../util/i18n.js';
import { initButtonSearch } from './buttonSearch.js';
import { initTabOutputs } from './tabOutputs.js';
import { showNotice } from './notice.js';

/** Un giro "de verdad" de rueda (un trackpad manda muchos deltas pequeños). */
const WHEEL_THRESHOLD = 40;
/** Como mucho un cambio de pestaña cada tanto: un giro rápido no las salta todas. */
const WHEEL_MIN_MS = 120;

let _config = null;
let _onRefresh = null;

export function initRadioFeatures(config, onRefresh) {
    _config = config;
    _onRefresh = onRefresh;
    applyBottomBar(config);
    initButtonSearch(() => _config, onRefresh);
    _wireTabWheel();
    initTabOutputs(() => _config, onRefresh);
    listen('audio-convert-status', e => {
        const { path, status, error } = e.payload ?? {};
        if (status !== 'failed') return;
        const name = String(path || '').split(/[\\/]/).pop();
        const key = `errors.${error}`;
        const reason = t(key) === key ? t('errors.audio_convert_failed') : t(key);
        showNotice(`${reason} — ${name}`, 'error', 8000);
    }).catch(console.error);
}

export function updateRadioFeatures(config) {
    _config = config;
    applyBottomBar(config);
}

/** Oculta la barra inferior si así se configuró. La rejilla (flex) ocupa el
 *  hueco sola y los botones se reajustan por el ResizeObserver de la rejilla. */
export function applyBottomBar(config) {
    document.body.classList.toggle('bottom-bar-hidden', config?.show_bottom_bar === false);
}

/**
 * Rueda vertical SOBRE LA BARRA DE PESTAÑAS: arriba = anterior, abajo =
 * siguiente. Fuera de esa zona la rueda no hace nada nuevo (la rejilla y el
 * panel siguen desplazándose como siempre). La rueda horizontal (Shift o
 * trackpad) sigue desplazando la barra si hay muchas pestañas.
 */
function _wireTabWheel() {
    const bar = document.querySelector('.tabs-container');
    if (!bar || bar.dataset.wheelReady) return;
    bar.dataset.wheelReady = '1';
    let accumulated = 0;
    let last = 0;
    let busy = false;
    bar.addEventListener('wheel', async e => {
        if (e.shiftKey || Math.abs(e.deltaY) <= Math.abs(e.deltaX)) return;
        e.preventDefault();
        accumulated += e.deltaY;
        const now = performance.now();
        if (busy || Math.abs(accumulated) < WHEEL_THRESHOLD || now - last < WHEEL_MIN_MS) return;
        const offset = accumulated > 0 ? 1 : -1;
        accumulated = 0;
        last = now;
        busy = true;
        try {
            await invoke('cycle_paleta', { offset });
            await _onRefresh?.();
        } catch (err) {
            console.error('cycle_paleta:', err);
        } finally {
            busy = false;
        }
    }, { passive: false });
}

/**
 * Archivo: radioFeatures.js
 * Propósito: arranque de las funciones de operación en directo, en un único
 * sitio para no engordar startup.js: buscador global, rueda del ratón sobre las
 * pestañas, barra inferior ocultable, avisos de salidas por pestaña y avisos de
 * conversión de M4A.
 */
import { invoke, listen } from '../bridge/api.js';
import { t } from '../util/i18n.js';
import { initButtonSearch, openButtonSearch } from './buttonSearch.js';
import { applyDisplayOptions } from './displayOptions.js';
import { initRelocate } from './relocate.js';
import { isMapping } from './mapping.js';
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
    applyDisplayOptions(config);
    initRelocate(onRefresh);
    initButtonSearch(() => _config, onRefresh);
    _wireTabWheel();
    _wireTabKey();
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
    applyDisplayOptions(config);
}

/**
 * La tecla Tab abre el buscador, lista para escribir el nombre del efecto. No se
 * toca si se está escribiendo en un campo, hay un diálogo abierto o se asigna un
 * atajo (Modo Mapeo): ahí Tab conserva su función de siempre.
 */
function _wireTabKey() {
    document.addEventListener('keydown', e => {
        if (e.key !== 'Tab' || e.ctrlKey || e.altKey || e.metaKey || e.shiftKey) return;
        if (isMapping() || _modalIsOpen()) return;
        const tag = e.target?.tagName;
        const inSearch = e.target?.closest?.('.button-search-panel');
        if (!inSearch && (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT')) return;
        e.preventDefault(); // sin esto el foco saltaria al siguiente control
        openButtonSearch();
    });
}

function _modalIsOpen() {
    return Boolean(document.querySelector('.modal-overlay:not(.hidden)'));
}

/** Oculta la barra inferior si así se configuró. La rejilla (flex) ocupa el
 *  hueco sola y los botones se reajustan por el ResizeObserver de la rejilla. */
export function applyBottomBar(config) {
    document.body.classList.toggle('bottom-bar-hidden', config?.show_bottom_bar === false);
}

/**
 * Rueda vertical SOBRE LA BARRA DE PESTAÑAS o SOBRE LA BOTONERA: arriba = anterior,
 * abajo = siguiente. Fuera de esas zonas la rueda no hace nada nuevo (el panel
 * fijo y las listas siguen desplazándose como siempre). La rueda horizontal
 * (Shift o trackpad) sigue desplazando la barra si hay muchas pestañas, y
 * Ctrl+rueda queda para el zoom.
 */
function _wireTabWheel() {
    let accumulated = 0;
    let last = 0;
    let busy = false;
    const onWheel = async e => {
        if (e.shiftKey || e.ctrlKey || Math.abs(e.deltaY) <= Math.abs(e.deltaX)) return;
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
    };
    for (const zone of [document.querySelector('.tabs-container'), document.getElementById('main-grid')]) {
        if (!zone || zone.dataset.wheelReady) continue;
        zone.dataset.wheelReady = '1';
        zone.addEventListener('wheel', onWheel, { passive: false });
    }
}

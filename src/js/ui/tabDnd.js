/**
 * Archivo: tabDnd.js
 * Proposito: reordenar pestanas arrastrandolas con el raton (sin tecla) o con
 * Alt+clic, y delegar el orden a Rust. La UI solo muestra origen/destino; el
 * orden real se valida y persiste en backend.
 */

import { invoke } from '../bridge/api.js';
import { appAlert } from './appDialog.js';
import { t } from '../util/i18n.js';
import { isMapping } from './mapping.js';
import { movedEnough, swallowNextClick } from '../util/dragGesture.js';

let _config = null;
let _onRefresh = null;
let _srcPaletaId = '';
let _pending = null; // pulsado sin Alt: aun no se sabe si es clic o arrastre
let _wired = false;

/** Conecta el arrastre de pestanas una sola vez y conserva config actual. */
export function initTabDnd(config, onRefresh) {
    updateTabDnd(config, onRefresh);
    if (_wired) return;
    _wired = true;
    _wire();
}

/** Actualiza dependencias usadas por el gesto tras cada refresco de config. */
export function updateTabDnd(config, onRefresh) {
    _config = config;
    _onRefresh = onRefresh;
}

function _wire() {
    document.addEventListener('mousedown', e => {
        if (e.button !== 0) return;
        const tab = _tabFrom(e.target);
        if (!tab) return;
        if (e.altKey) {
            _begin(tab);
            e.preventDefault();
            return;
        }
        if (e.ctrlKey || e.shiftKey || e.metaKey || isMapping()) return;
        _pending = { x: e.clientX, y: e.clientY, tab };
    });

    document.addEventListener('mousemove', e => {
        // Se solto el boton fuera de la ventana: no llego ningun mouseup.
        if ((_srcPaletaId || _pending) && e.buttons === 0) {
            _pending = null;
            _srcPaletaId = '';
            _clearMarks();
            return;
        }
        if (_pending && movedEnough(_pending, e)) {
            _begin(_pending.tab);
            _pending = null;
        }
        if (!_srcPaletaId) return;
        _markTarget(_tabFrom(e.target));
    });

    document.addEventListener('mouseup', e => {
        _pending = null; // fue un clic: cambia de pestana como siempre
        if (!_srcPaletaId) return;
        swallowNextClick(); // soltar tras arrastrar no debe activar la pestana
        const fromPaletaId = _srcPaletaId;
        const toPaletaId = _tabFrom(e.target)?.dataset.paletaId ?? '';
        _srcPaletaId = '';
        _clearMarks();
        void _drop(fromPaletaId, toPaletaId);
    });
}

function _begin(tab) {
    _srcPaletaId = tab.dataset.paletaId;
    tab.classList.add('tab-drag-source');
}

async function _drop(fromPaletaId, toPaletaId) {
    if (!toPaletaId || toPaletaId === fromPaletaId) return;
    try {
        await invoke('reorder_paletas', {
            profileId: _config.active_profile_id,
            fromPaletaId,
            toPaletaId,
        });
        _onRefresh?.();
    } catch (err) {
        const key = `errors.${err}`;
        const msg = t(key);
        await appAlert(msg === key ? String(err) : msg);
    }
}

function _markTarget(tab) {
    document.querySelectorAll('.tab.tab-drag-over')
        .forEach(el => el.classList.remove('tab-drag-over'));
    if (tab?.dataset.paletaId !== _srcPaletaId) {
        tab?.classList.add('tab-drag-over');
    }
}

function _clearMarks() {
    document.querySelectorAll('.tab.tab-drag-source, .tab.tab-drag-over')
        .forEach(el => el.classList.remove('tab-drag-source', 'tab-drag-over'));
}

function _tabFrom(target) {
    return target.closest?.('#tabs-list .tab[data-paleta-id]') ?? null;
}

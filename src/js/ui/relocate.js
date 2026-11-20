/**
 * Archivo: relocate.js
 * Propósito: cuando un efecto no se encuentra (borrado, movido, o la app instalada
 * en otra PC), ofrecer elegir una carpeta donde buscarlo. Rust recorre la carpeta,
 * empareja por nombre y corrige la ruta de TODOS los efectos perdidos a la vez
 * (cmd_relocate.rs): al instalar en otra PC basta hacerlo una vez.
 */
import { invoke } from '../bridge/api.js';
import { t } from '../util/i18n.js';
import { appConfirm } from './appDialog.js';
import { showNotice } from './notice.js';

let _onRefresh = null;
let _busy = false;

export function initRelocate(onRefresh) {
    _onRefresh = onRefresh;
}

/** Pregunta, deja elegir carpeta y reubica. Un solo diálogo a la vez: si se
 *  pulsan varios botones perdidos seguidos no se apilan las preguntas. */
export async function offerRelocate() {
    if (_busy) return;
    _busy = true;
    try {
        const wants = await appConfirm(
            t('relocate.body'),
            { ok: t('relocate.search'), cancel: t('edit_modal.cancel') },
            { ok: 1, cancel: 2 },
        );
        if (!wants) return;
        const folder = await invoke('pick_named_folder').catch(() => null);
        if (!folder?.path) return; // canceló el selector de carpetas
        showNotice(t('relocate.searching'), 'info', 2500);
        const result = await invoke('relocate_missing_files', { folder: folder.path });
        showNotice(_summary(result), result.relocated ? 'info' : 'error', 7000);
        if (result.relocated) await _onRefresh?.();
    } catch (error) {
        console.error('relocate:', error);
        const key = `errors.${error}`;
        showNotice(t(key) === key ? String(error) : t(key), 'error', 7000);
    } finally {
        _busy = false;
    }
}

function _summary({ relocated, still_missing: stillMissing }) {
    if (!relocated) return t('relocate.none');
    const found = t('relocate.found').replace('{n}', relocated);
    return stillMissing
        ? `${found} ${t('relocate.still').replace('{n}', stillMissing)}`
        : found;
}

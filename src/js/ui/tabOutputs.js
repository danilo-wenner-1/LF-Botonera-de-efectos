/**
 * Archivo: tabOutputs.js
 * Propósito: avisar cuando la salida de audio propia de una pestaña no está
 * disponible y ofrecer una alternativa. Rust decide el ruteo (cmd_tab_outputs)
 * y ya hace sonar esas pestañas por la salida principal; aquí solo se informa.
 */
import { invoke } from '../bridge/api.js';
import { t } from '../util/i18n.js';

const POLL_MS = 4000;

let _getConfig = () => null;
let _onRefresh = null;
let _modal = null;
let _timer = null;
let _shown = '';      // firma de los avisos ya mostrados (no repetir el mismo)
let _busy = false;

/** Arranca la vigilancia. `getConfig` devuelve la AppConfig vigente. */
export function initTabOutputs(getConfig, onRefresh) {
    _getConfig = getConfig;
    _onRefresh = onRefresh;
    if (!_timer) _timer = setInterval(() => _poll(), POLL_MS);
    window.addEventListener('focus', () => checkTabOutputs());
    return checkTabOutputs();
}

/** Reaplica el ruteo y muestra los avisos nuevos. Devuelve el estado. */
export async function checkTabOutputs() {
    if (_busy) return null;
    _busy = true;
    try {
        const status = await invoke('get_tab_output_status');
        _report(status);
        return status;
    } catch (e) {
        console.error('tab outputs:', e);
        return null;
    } finally {
        _busy = false;
    }
}

/** Solo vigila si alguna pestaña del perfil tiene salida propia: sin ellas no
 *  hay nada que pueda desconectarse y no se gasta ni una consulta. */
function _poll() {
    const cfg = _getConfig();
    const profile = cfg?.profiles?.find(p => p.id === cfg.active_profile_id);
    if (profile?.paletas?.some(p => (p.audio_out || '').trim())) checkTabOutputs();
}

function _report(status) {
    const issues = status?.issues ?? [];
    const signature = issues.map(i => `${i.paleta_id}|${i.device}|${i.reason}`).join(';');
    if (!issues.length) {
        _shown = '';
        _modal?.classList.add('hidden');
        return;
    }
    if (signature === _shown) return;
    _shown = signature;
    _show(issues, status.devices ?? []);
}

function _show(issues, devices) {
    _ensureModal();
    const list = _modal.querySelector('.tab-output-list');
    list.innerHTML = '';
    issues.forEach(issue => list.appendChild(_row(issue, devices)));
    _modal.classList.remove('hidden');
}

function _row(issue, devices) {
    const row = document.createElement('div');
    row.className = 'tab-output-issue';
    const text = document.createElement('p');
    text.textContent = `${issue.nombre}: «${issue.device}» — ${t(`tab_outputs.${issue.reason}`)}`;
    const select = document.createElement('select');
    select.className = 'audio-select';
    select.appendChild(_option('', t('tab_modal.audio_out_main')));
    devices.forEach(d => select.appendChild(_option(d, d)));
    const apply = document.createElement('button');
    apply.className = 'btn-blue';
    apply.textContent = t('tab_outputs.use_selected');
    apply.addEventListener('click', () => _assign(issue.paleta_id, select.value, row));
    row.append(text, select, apply);
    return row;
}

function _option(value, label) {
    const opt = document.createElement('option');
    opt.value = value;
    opt.textContent = label;
    return opt;
}

async function _assign(paletaId, device, row) {
    const cfg = _getConfig();
    try {
        await invoke('set_tab_output', {
            profileId: cfg.active_profile_id, paletaId, device,
        });
        row.remove();
        if (!_modal.querySelector('.tab-output-issue')) _modal.classList.add('hidden');
        _shown = '';
        _onRefresh?.();
    } catch (e) {
        console.error('set_tab_output:', e);
    }
}

function _ensureModal() {
    if (_modal) return;
    _modal = document.createElement('div');
    _modal.className = 'modal-overlay hidden';
    _modal.style.zIndex = '4800';
    _modal.innerHTML = `
      <div class="modal-content modal-sm">
        <div class="modal-header"><h3></h3></div>
        <div class="modal-body">
          <p class="confirm-text tab-output-intro"></p>
          <div class="tab-output-list"></div>
        </div>
        <div class="modal-footer">
          <button class="btn-dark tab-output-all-main"></button>
          <button class="btn-dark tab-output-keep"></button>
        </div>
      </div>`;
    _modal.querySelector('h3').textContent = t('tab_outputs.title');
    _modal.querySelector('.tab-output-intro').textContent = t('tab_outputs.body');
    const allMain = _modal.querySelector('.tab-output-all-main');
    allMain.textContent = t('tab_outputs.use_main');
    allMain.addEventListener('click', () => {
        _modal.querySelectorAll('.tab-output-issue select').forEach(s => { s.value = ''; });
        _modal.querySelectorAll('.tab-output-issue .btn-blue').forEach(b => b.click());
    });
    const keep = _modal.querySelector('.tab-output-keep');
    keep.textContent = t('tab_outputs.keep');
    keep.addEventListener('click', () => _modal.classList.add('hidden'));
    document.body.appendChild(_modal);
}

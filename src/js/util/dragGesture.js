/**
 * Archivo: dragGesture.js
 * Propósito: lo común del arrastre con el ratón sin tecla modificadora. Un arrastre
 * solo empieza cuando el puntero se mueve unos píxeles con el botón pulsado: un
 * clic normal sigue disparando el efecto. Al soltar tras un arrastre se traga el
 * clic que el navegador genera, para que soltar un botón sobre sí mismo (o sobre
 * otro) no lo reproduzca.
 */

/** Píxeles que hay que mover antes de considerarlo un arrastre. */
export const DRAG_THRESHOLD = 6;

let _swallow = false;
let _timer = null;

// Fase de captura en el documento: gana a los manejadores de clic de botones y
// pestañas, que se registran más abajo.
document.addEventListener('click', e => {
    if (!_swallow) return;
    _swallow = false;
    e.stopPropagation();
    e.preventDefault();
}, true);

/** Ignora el próximo clic. Se desactiva solo: si el navegador no llega a generar
 *  clic (se soltó fuera de la ventana), no debe comerse el siguiente de verdad. */
export function swallowNextClick() {
    _swallow = true;
    clearTimeout(_timer);
    _timer = setTimeout(() => { _swallow = false; }, 120);
}

/** ¿Se ha movido el puntero lo bastante desde `origin` ({x, y})? */
export function movedEnough(origin, e) {
    return Math.hypot(e.clientX - origin.x, e.clientY - origin.y) > DRAG_THRESHOLD;
}

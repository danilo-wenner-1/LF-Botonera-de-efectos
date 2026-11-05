//! Modulo: engine/console/thread.rs
//! Proposito: el hilo guardian de la consola. Unico dueno de las tarjetas
//! abiertas (`EndpointRegistry`), porque `OutputStream` no es Send y tiene que
//! quedarse quieto en algun sitio. Solo atiende cambios de ruteo: reproducir NO
//! pasa por aqui — los motores anaden fuentes al bus desde sus propios hilos.
use super::endpoint::EndpointRegistry;
use super::graph;
use super::{BusId, ConsoleCommand, ConsoleState, Routing};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};

pub fn run(rx: Receiver<ConsoleCommand>, state: Arc<Mutex<ConsoleState>>) {
    let mut endpoints = EndpointRegistry::default();
    for cmd in rx {
        // `reopen`: un ruteo igual pero con el bus caido, ¿se reintenta? Si para
        // un bus suelto (arranque, reconexion de la principal). NO para los
        // carriles de pestaña: la UI los reaplica cada pocos segundos, y una
        // tarjeta que figura pero no abre (otro programa la tiene en exclusivo)
        // reharia el grafo —y cortaria todo lo que suena— en cada consulta.
        // Mientras tanto sus botones suenan por la principal (`routing::resolve`).
        let (routings, done, reopen) = match cmd {
            ConsoleCommand::SetBusRouting { bus, routing, done } => {
                (vec![(bus, routing)], done, true)
            }
            ConsoleCommand::SetRoutings { routings, done } => (routings, done, false),
        };
        if apply(&state, routings, reopen) {
            graph::rebuild(&mut endpoints, &state);
        }
        notify(done);
    }
}

/// Guarda los ruteos pedidos y dice si hay que rehacer el grafo.
///
/// Rehacer el grafo mata las fuentes que estuvieran sonando, asi que no se hace
/// por gusto: reaplicar el mismo ruteo (al arrancar, al reconectar) no debe
/// tocar nada. Que un bus que deberia existir no exista si obliga: hay que
/// abrirlo. Un carril de pestaña sin tarjeta no "deberia existir".
fn apply(state: &Arc<Mutex<ConsoleState>>, routings: Vec<(BusId, Routing)>, reopen: bool) -> bool {
    let mut guard = state.lock().unwrap();
    let mut changed = false;
    for (bus, routing) in routings {
        let live = guard.live.contains_key(&bus);
        let Some(slot) = guard.slots.get_mut(&bus) else {
            continue;
        };
        let same = slot.routing == routing;
        let satisfied = live || !bus.wants_output(&routing) || !reopen;
        if !(same && satisfied) {
            slot.routing = routing;
            changed = true;
        }
    }
    changed
}

/// Se avisa siempre, tambien cuando no hubo nada que hacer: quien espera
/// necesita seguir, no quedarse colgado hasta el tope.
fn notify(done: Option<SyncSender<()>>) {
    if let Some(done) = done {
        let _ = done.send(());
    }
}

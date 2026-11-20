//! Modulo: domain/button/relocate.rs
//! Proposito: encontrar un audio que ya no esta donde decia la botonera (se
//! borro, se movio o se instalo la app en otra PC) buscandolo por su nombre en una
//! carpeta que elige el usuario. Pura: recibe la lista de archivos de la carpeta,
//! no toca el disco.
//!
//! Si el mismo nombre aparece en varias subcarpetas gana la que mas se parece a
//! la ruta original: se comparan los componentes del final de la ruta (carpeta
//! contenedora, la de arriba...). Asi `Efectos/Aplausos/a1.mp3` encuentra su sitio
//! aunque exista otro `a1.mp3` en `Cortinas/`.
use std::collections::HashMap;
use std::path::Path;

/// Clave de comparacion de un nombre de archivo: sin mayusculas (Windows no las
/// distingue) y con la ruta separada por `/` o `\`, sea cual sea el sistema.
fn file_key(path: &str) -> String {
    components(path).last().cloned().unwrap_or_default()
}

fn components(path: &str) -> Vec<String> {
    path.split(['/', '\\'])
        .filter(|part| !part.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Cuantos componentes consecutivos, contados desde el final, comparten dos rutas.
fn shared_tail(a: &[String], b: &[String]) -> usize {
    a.iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(x, y)| x == y)
        .count()
}

/// El archivo de `candidates` que mejor sustituye a `original`. None si ninguno
/// se llama igual.
pub fn best_match<'a>(original: &str, candidates: &'a [String]) -> Option<&'a String> {
    let key = file_key(original);
    if key.is_empty() {
        return None;
    }
    let wanted = components(original);
    candidates
        .iter()
        .filter(|candidate| file_key(candidate) == key)
        // A igualdad, la ruta mas corta y luego la menor alfabeticamente: el
        // resultado no depende del orden en que el disco enumero la carpeta.
        .max_by(|a, b| {
            shared_tail(&wanted, &components(a))
                .cmp(&shared_tail(&wanted, &components(b)))
                .then(b.len().cmp(&a.len()))
                .then(b.cmp(a))
        })
}

/// Para cada ruta que falta, la que la sustituye. Las que no aparecen en la
/// carpeta se omiten (siguen perdidas).
pub fn plan(missing: &[String], candidates: &[String]) -> HashMap<String, String> {
    // Se indexa una vez por nombre: con cientos de efectos perdidos y miles de
    // archivos, comparar todos contra todos seria cuadratico.
    let mut by_name: HashMap<String, Vec<String>> = HashMap::new();
    for candidate in candidates {
        by_name
            .entry(file_key(candidate))
            .or_default()
            .push(candidate.clone());
    }
    missing
        .iter()
        .filter_map(|original| {
            let pool = by_name.get(&file_key(original))?;
            best_match(original, pool).map(|found| (original.clone(), found.clone()))
        })
        .collect()
}

/// Ruta que no existe en disco. Una ruta vacia no cuenta: es un boton sin audio.
pub fn is_missing(path: &str) -> bool {
    !path.trim().is_empty() && !Path::new(path).exists()
}

#[cfg(test)]
#[path = "relocate_tests.rs"]
mod relocate_tests;

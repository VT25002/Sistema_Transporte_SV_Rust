use std::collections::{HashMap, VecDeque};
use petgraph::graph::{NodeIndex, UnGraph};
use crate::modelos::{Departamento, Carretera};

pub fn bfs_ruta(
    grafo: &UnGraph<Departamento, Carretera>,
    nodos: &Vec<NodeIndex>,
    origen: &str,
    destino: &str,
) -> Option<(Vec<String>, f64)> {

    // Buscar índices de origen y destino por nombre
    let nodo_origen = nodos.iter().find(|&&n| {
        grafo[n].nombre.to_lowercase() == origen.to_lowercase()
    });

    let nodo_destino = nodos.iter().find(|&&n| {
        grafo[n].nombre.to_lowercase() == destino.to_lowercase()
    });

    let (inicio, fin) = match (nodo_origen, nodo_destino) {
        (Some(&i), Some(&f)) => (i, f),
        _ => return None, // Si no existe alguno de los dos, retorna None
    };

    // Caso especial: origen y destino son el mismo
    if inicio == fin {
        return Some((vec![grafo[inicio].nombre.clone()], 0.0));
    }

    // BFS
    let mut visitados: HashMap<NodeIndex, Option<NodeIndex>> = HashMap::new();
    let mut cola: VecDeque<NodeIndex> = VecDeque::new();

    visitados.insert(inicio, None);
    cola.push_back(inicio);

    while let Some(actual) = cola.pop_front() {
        if actual == fin {
            break;
        }

        for vecino in grafo.neighbors(actual) {
            if !visitados.contains_key(&vecino) {
                visitados.insert(vecino, Some(actual));
                cola.push_back(vecino);
            }
        }
    }

    // Reconstruir el camino desde destino hacia origen
    if !visitados.contains_key(&fin) {
        return None; // No hay camino
    }

    let mut camino: Vec<String> = Vec::new();
    let mut actual = fin;

    loop {
        camino.push(grafo[actual].nombre.clone());
        match visitados[&actual] {
            None => break,
            Some(anterior) => actual = anterior,
        }
    }

    camino.reverse();

    // Calcular kilómetros totales del camino encontrado
    let mut km_totales = 0.0;
    for i in 0..camino.len() - 1 {
        let a = nodos.iter().find(|&&n| grafo[n].nombre == camino[i]).unwrap();
        let b = nodos.iter().find(|&&n| grafo[n].nombre == camino[i + 1]).unwrap();
        if let Some(arista) = grafo.find_edge(*a, *b) {
            km_totales += grafo[arista].distancia_km;
        }
    }

    Some((camino, km_totales))
}
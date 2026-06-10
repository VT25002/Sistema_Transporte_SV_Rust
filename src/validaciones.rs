use petgraph::graph::UnGraph;
use crate::modelos::{Departamento, Carretera};
use petgraph::graph::NodeIndex;

// Lista oficial de departamentos válidos
pub fn departamentos_validos() -> Vec<&'static str> {
    vec![
        "Ahuachapán", "Santa Ana", "Sonsonate",
        "Chalatenango", "La Libertad", "San Salvador",
        "Cuscatlán", "La Paz", "Cabañas", "San Vicente",
        "Usulután", "San Miguel", "Morazán", "La Unión",
    ]
}

// Verifica si un nombre de departamento es válido (sin importar mayúsculas)
pub fn es_departamento_valido(nombre: &str) -> bool {
    departamentos_validos()
        .iter()
        .any(|d| d.to_lowercase() == nombre.to_lowercase())
}

// Busca el NodeIndex de un departamento por nombre en el grafo
pub fn buscar_nodo<'a>(
    grafo: &'a UnGraph<Departamento, Carretera>,
    nodos: &'a Vec<NodeIndex>,
    nombre: &str,
) -> Option<NodeIndex> {
    nodos.iter().find(|&&n| {
        grafo[n].nombre.to_lowercase() == nombre.to_lowercase()
    }).copied()
}

// Normaliza el nombre: quita espacios al inicio y al final
pub fn normalizar_entrada(entrada: &str) -> String {
    entrada.trim().to_string()
}

// =============================================
// TESTS AUTOMATIZADOS
// =============================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grafo::inicializar_mapa;
    use crate::algoritmo::bfs_ruta;

    #[test]
    fn test_departamento_valido() {
        assert!(es_departamento_valido("San Salvador"));
        assert!(es_departamento_valido("san salvador")); // minúsculas
        assert!(es_departamento_valido("SAN SALVADOR")); // mayúsculas
    }

    #[test]
    fn test_departamento_invalido() {
        assert!(!es_departamento_valido("San Josecito"));
        assert!(!es_departamento_valido(""));
        assert!(!es_departamento_valido("Guatemala"));
    }

    #[test]
    fn test_normalizar_entrada() {
        assert_eq!(normalizar_entrada("  San Salvador  "), "San Salvador");
        assert_eq!(normalizar_entrada("La Paz"), "La Paz");
    }

    #[test]
    fn test_bfs_ruta_existe() {
        let (grafo, nodos) = inicializar_mapa();
        let resultado = bfs_ruta(&grafo, &nodos, "Ahuachapán", "San Salvador");
        assert!(resultado.is_some());
    }

    #[test]
    fn test_bfs_ruta_misma_ciudad() {
        let (grafo, nodos) = inicializar_mapa();
        let resultado = bfs_ruta(&grafo, &nodos, "San Salvador", "San Salvador");
        assert!(resultado.is_some());
        let (camino, km) = resultado.unwrap();
        assert_eq!(camino.len(), 1);
        assert_eq!(km, 0.0);
    }

    #[test]
    fn test_bfs_ruta_no_existe() {
        let (grafo, nodos) = inicializar_mapa();
        let resultado = bfs_ruta(&grafo, &nodos, "Ahuachapán", "Narnia");
        assert!(resultado.is_none());
    }

    #[test]
    fn test_bfs_menos_escalas_occidental_a_oriental() {
        let (grafo, nodos) = inicializar_mapa();
        let resultado = bfs_ruta(&grafo, &nodos, "Ahuachapán", "La Unión");
        assert!(resultado.is_some());
        let (camino, _) = resultado.unwrap();
        // El camino debe tener más de 2 pasos
        assert!(camino.len() > 2);
    }
}
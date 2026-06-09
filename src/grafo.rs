use crate::modelos::{Carretera, Departamento, Zona};
use petgraph::graph::UnGraph;

pub fn inicializar_mapa() -> (
    UnGraph<Departamento, Carretera>,
    Vec<petgraph::graph::NodeIndex>,
) {
    let mut grafo = UnGraph::<Departamento, Carretera>::new_undirected();

    // === NODOS (14 departamentos) ===
    let ahuachapan = grafo.add_node(Departamento::new(0, "Ahuachapán", Zona::Occidental));
    let santa_ana = grafo.add_node(Departamento::new(1, "Santa Ana", Zona::Occidental));
    let sonsonate = grafo.add_node(Departamento::new(2, "Sonsonate", Zona::Occidental));
    let chalatenango = grafo.add_node(Departamento::new(3, "Chalatenango", Zona::Central));
    let la_libertad = grafo.add_node(Departamento::new(4, "La Libertad", Zona::Central));
    let san_salvador = grafo.add_node(Departamento::new(5, "San Salvador", Zona::Central));
    let cuscatlan = grafo.add_node(Departamento::new(6, "Cuscatlán", Zona::Central));
    let la_paz = grafo.add_node(Departamento::new(7, "La Paz", Zona::Paracentral));
    let cabanas = grafo.add_node(Departamento::new(8, "Cabañas", Zona::Paracentral));
    let san_vicente = grafo.add_node(Departamento::new(9, "San Vicente", Zona::Paracentral));
    let usulutan = grafo.add_node(Departamento::new(10, "Usulután", Zona::Oriental));
    let san_miguel = grafo.add_node(Departamento::new(11, "San Miguel", Zona::Oriental));
    let morazan = grafo.add_node(Departamento::new(12, "Morazán", Zona::Oriental));
    let la_union = grafo.add_node(Departamento::new(13, "La Unión", Zona::Oriental));

    // === ARISTAS (carreteras con distancias en KM) ===

    // Zona Occidental
    grafo.add_edge(ahuachapan, santa_ana, Carretera::new(35.0));
    grafo.add_edge(ahuachapan, sonsonate, Carretera::new(45.0));
    grafo.add_edge(santa_ana, sonsonate, Carretera::new(40.0));

    // Occidental -> Central
    grafo.add_edge(santa_ana, chalatenango, Carretera::new(75.0));
    grafo.add_edge(santa_ana, la_libertad, Carretera::new(80.0));
    grafo.add_edge(sonsonate, la_libertad, Carretera::new(65.0));

    // Zona Central / Paracentral
    grafo.add_edge(chalatenango, san_salvador, Carretera::new(50.0));
    grafo.add_edge(la_libertad, san_salvador, Carretera::new(30.0));
    grafo.add_edge(la_libertad, la_paz, Carretera::new(45.0));
    grafo.add_edge(san_salvador, cuscatlan, Carretera::new(35.0));
    grafo.add_edge(san_salvador, la_paz, Carretera::new(40.0));
    grafo.add_edge(cuscatlan, cabanas, Carretera::new(40.0));
    grafo.add_edge(cuscatlan, san_vicente, Carretera::new(45.0));
    grafo.add_edge(la_paz, san_vicente, Carretera::new(55.0));
    grafo.add_edge(cabanas, san_vicente, Carretera::new(35.0));

    // Central -> Oriental
    grafo.add_edge(san_vicente, usulutan, Carretera::new(60.0));

    // Zona Oriental
    grafo.add_edge(usulutan, san_miguel, Carretera::new(50.0));
    grafo.add_edge(san_miguel, morazan, Carretera::new(45.0));
    grafo.add_edge(san_miguel, la_union, Carretera::new(65.0));
    grafo.add_edge(morazan, la_union, Carretera::new(70.0));

    // Devolvemos el grafo y los índices de los nodos
    let nodos = vec![
        ahuachapan,
        santa_ana,
        sonsonate,
        chalatenango,
        la_libertad,
        san_salvador,
        cuscatlan,
        la_paz,
        cabanas,
        san_vicente,
        usulutan,
        san_miguel,
        morazan,
        la_union,
    ];

    (grafo, nodos)
}

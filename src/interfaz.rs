use std::io::{self, Write};
use crate::{grafo, algoritmo, validaciones};

pub fn ejecutar_sistema() {

    println!("\n>>> Cargando la base de datos con las conexiones entre rutas...");
    let (mapa, nodos) = grafo::inicializar_mapa();
    println!(">>> Mapa cargado con éxito, 14 departamentos en línea.\n");

    loop {
        println!("+-------------------------------------------------------+");
        println!("|         SISTEMA DE TRANSPORTE 'RUTASV' - SV           |");
        println!("+-------------------------------------------------------+");
        println!("|   [1] Buscar ruta óptima (Menos escalas)              |");
        println!("|   [2] Listar departamentos del mapa                   |");
        println!("|   [3] Salir del sistema                               |");
        println!("+-------------------------------------------------------+");
        print!("\nSeleccione una opción (1-3) -> ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
            "1" => {
                limpiar_pantalla();
                procesar_busqueda(&mapa, &nodos);
            }
            "2" => {
                limpiar_pantalla();
                mostrar_departamentos();
            }
            "3" => {
                println!("\n>>> Finalizando ejecución. ¡Gracias por usar 'RutaSV', buen viaje!");
                break;
            }
            _ => {
                limpiar_pantalla();
                println!("[!] Opción inválida. Utilice los identificadores [1], [2] o [3].\n");
            }
        }
    }
}

fn procesar_busqueda(
    mapa: &petgraph::graph::UnGraph<crate::modelos::Departamento, crate::modelos::Carretera>,
    nodos: &Vec<petgraph::graph::NodeIndex>,
) {
    println!("+--------------------------------------------------+");
    println!("|              MODULO DE BÚSQUEDA                  |");
    println!("+--------------------------------------------------+");
    
    let origen = pedir_departamento("ORIGEN");
    let destino = pedir_departamento("DESTINO");

    println!("\n[Procesando] Rastreando grafo de adyacencia...");

    match algoritmo::bfs_ruta(mapa, nodos, &origen, &destino) {
        Some((camino, km_totales)) => {
            println!("\n================ RESULTADO DE RUTA ================");
            
            // Usamos flechas de caracteres simples, más minimalistas
            let ruta_formateada = camino.join(" -> ");
            println!("  Trayecto: {}", ruta_formateada);
            println!("  Distancia total: {:.1} km", km_totales);
            
            println!("===================================================\n");
        }
        None => {
            println!("\n[!] ERROR: No existe una conexión vial directa registrada.\n");
        }
    }
    
    // Pausa para que el usuario pueda leer el resultado antes de que el menú limpie la pantalla
    enter_para_continuar();
    limpiar_pantalla();
}

// Función auxiliar para capturar teclado
fn pedir_departamento(tipo: &str) -> String {
    loop {
        print!("  -> Ingrese punto de {}: ", tipo);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        
        let entrada_limpia = validaciones::normalizar_entrada(&entrada);

        if validaciones::es_departamento_valido(&entrada_limpia) {
            return entrada_limpia;
        } else {
            println!("     [!] Entrada inválida: '{}' no coincide con el mapa.", entrada_limpia);
        }
    }
}

// Función para listar y ayudar al usuario si no sabe cómo escribir los departamentos
fn mostrar_departamentos() {
    println!("+--------------------------------------------------+");
    println!("|        DEPARTAMENTOS REGISTRADOS EN EL MAPA      |");
    println!("+--------------------------------------------------+");
    
    let lista = validaciones::departamentos_validos();
    // Se imprimen en dos columnas para que no ocupe tanto espacio vertical
    for chunk in lista.chunks(2) {
        if chunk.len() == 2 {
            println!("  * {:<20} * {:<20}", chunk[0], chunk[1]);
        } else {
            println!("  * {:<20}", chunk[0]);
        }
    }
    println!("+--------------------------------------------------+\n");
    
    enter_para_continuar();
    limpiar_pantalla();
}

// Función que permite limpiar la pantalla
fn limpiar_pantalla() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

// Función para hacer pausa en la interfaz
fn enter_para_continuar() {
    print!("Presione [ENTER] para regresar al menú principal...");
    io::stdout().flush().unwrap();
    let mut _basura = String::new();
    io::stdin().read_line(&mut _basura).unwrap();
}
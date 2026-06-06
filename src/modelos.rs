#[derive(Debug, Clone, PartialEq)]
pub enum Zona {
    Occidental,
    Central,
    Paracentral,
    Oriental,
}

#[derive(Debug, Clone)]
pub struct Departamento {
    pub id: u32,
    pub nombre: String,
    pub zona: Zona,
}

impl Departamento {
    pub fn new(id: u32, nombre: &str, zona: Zona) -> Self {
        Self {
            id,
            nombre: nombre.to_string(),
            zona,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Carretera {
    pub distancia_km: f64,
}

impl Carretera {
    pub fn new(distancia_km: f64) -> Self {
        Self { distancia_km }
    }
}
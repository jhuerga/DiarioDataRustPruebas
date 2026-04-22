use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;

// Tipo de documento: valores controlados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipoDocumento {
    Dni,
    Nie,
    Pasaporte,
    Otro,
}

impl FromStr for TipoDocumento {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dni" => Ok(TipoDocumento::Dni),
            "Nie" => Ok(TipoDocumento::Nie),
            "Pasaporte" => Ok(TipoDocumento::Pasaporte),
            "Otro" => Ok(TipoDocumento::Otro),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TipoDocumento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TipoDocumento::Dni => "Dni",
            TipoDocumento::Nie => "Nie",
            TipoDocumento::Pasaporte => "Pasaporte",
            TipoDocumento::Otro => "Otro",
        };
        write!(f, "{}", s)
    }
}

impl TipoDocumento {
    pub fn all_as_pairs() -> Vec<(String, String)> {
        vec![
            ("Dni".to_string(), "DNI".to_string()),
            ("Nie".to_string(), "NIE".to_string()),
            ("Pasaporte".to_string(), "Pasaporte".to_string()),
            ("Otro".to_string(), "Otro".to_string()),
        ]
    }
}

// Sexo: valores controlados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sexo {
    Varon,
    Mujer,
    NA,
}

impl FromStr for Sexo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Varon" => Ok(Sexo::Varon),
            "Mujer" => Ok(Sexo::Mujer),
            "NA" => Ok(Sexo::NA),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Sexo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Sexo::Varon => "Varon",
            Sexo::Mujer => "Mujer",
            Sexo::NA => "NA",
        };
        write!(f, "{}", s)
    }
}

impl Sexo {
    pub fn all_as_pairs() -> Vec<(String, String)> {
        vec![
            ("Varon".to_string(), "Varón".to_string()),
            ("Mujer".to_string(), "Mujer".to_string()),
            ("NA".to_string(), "N/A".to_string()),
        ]
    }
}

// Estado civil: valores controlados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EstadoCivil {
    Soltero,
    Casado,
    Viudo,
    Divorciado,
    ParejaDeHecho,
    Desconocido,
}

impl FromStr for EstadoCivil {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Soltero" => Ok(EstadoCivil::Soltero),
            "Casado" => Ok(EstadoCivil::Casado),
            "Viudo" => Ok(EstadoCivil::Viudo),
            "Divorciado" => Ok(EstadoCivil::Divorciado),
            "ParejaDeHecho" => Ok(EstadoCivil::ParejaDeHecho),
            "Desconocido" => Ok(EstadoCivil::Desconocido),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EstadoCivil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EstadoCivil::Soltero => "Soltero",
            EstadoCivil::Casado => "Casado",
            EstadoCivil::Viudo => "Viudo",
            EstadoCivil::Divorciado => "Divorciado",
            EstadoCivil::ParejaDeHecho => "ParejaDeHecho",
            EstadoCivil::Desconocido => "Desconocido",
        };
        write!(f, "{}", s)
    }
}

impl EstadoCivil {
    pub fn all_as_pairs() -> Vec<(String, String)> {
        vec![
            ("Soltero".to_string(), "Soltero/a".to_string()),
            ("Casado".to_string(), "Casado/a".to_string()),
            ("Viudo".to_string(), "Viudo/a".to_string()),
            ("Divorciado".to_string(), "Divorciado/a".to_string()),
            ("ParejaDeHecho".to_string(), "Pareja de hecho".to_string()),
            ("Desconocido".to_string(), "Desconocido".to_string()),
        ]
    }
}

// Provincia: valores controlados (ejemplo, agregar más según necesidad)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Provincia {
    Madrid,
    Barcelona,
    Valencia,
    Sevilla,
    Bilbao,
    Otro,
}

impl FromStr for Provincia {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Madrid" => Ok(Provincia::Madrid),
            "Barcelona" => Ok(Provincia::Barcelona),
            "Valencia" => Ok(Provincia::Valencia),
            "Sevilla" => Ok(Provincia::Sevilla),
            "Bilbao" => Ok(Provincia::Bilbao),
            "Otro" => Ok(Provincia::Otro),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Provincia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Provincia::Madrid => "Madrid",
            Provincia::Barcelona => "Barcelona",
            Provincia::Valencia => "Valencia",
            Provincia::Sevilla => "Sevilla",
            Provincia::Bilbao => "Bilbao",
            Provincia::Otro => "Otro",
        };
        write!(f, "{}", s)
    }
}

impl Provincia {
    pub fn all_as_pairs() -> Vec<(String, String)> {
        vec![
            ("Madrid".to_string(), "Madrid".to_string()),
            ("Barcelona".to_string(), "Barcelona".to_string()),
            ("Valencia".to_string(), "Valencia".to_string()),
            ("Sevilla".to_string(), "Sevilla".to_string()),
            ("Bilbao".to_string(), "Bilbao".to_string()),
            ("Otro".to_string(), "Otro".to_string()),
        ]
    }
}

//
// ============================================================
//  SECCIONES DEL FORMULARIO
// ============================================================
//

// ------------------------------------------------------------
// IDENTIFICACIÓN
// ------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identificacion {
    pub dni: String,                     // Validación DNI/NIE/PAS → aviso no bloqueante
    pub codigo: String,                  // Código interno único
    pub tipo_documento: TipoDocumento,   // Enum
    pub apellidos: String,
    pub nombre: String,
    pub naf: String,                     // Validación NAF → aviso no bloqueante
    pub fecha_caducidad_documento: Option<String>, // YYYY-MM-DD
}

// ------------------------------------------------------------
// DATOS PERSONALES
// ------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatosPersonales {
    pub fecha_nacimiento: Option<String>, // YYYY-MM-DD
    pub edad: Option<u16>,                // Campo derivado → se calcula al perder foco
    pub sexo: Sexo,                       // Enum
    pub nacionalidad_iso3: Option<String>,// FK a TblPaises
    pub estado_civil: EstadoCivil,        // Enum
}

// ------------------------------------------------------------
// DATOS FISCALES
// ------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatosFiscales {
    pub cp: String,
    pub domicilio: String,
    pub poblacion: String,
    pub provincia_cod: Option<String>, // FK a TblProvincias
    pub iban: String,                  // Validación IBAN → aviso no bloqueante
}

// ------------------------------------------------------------
// CONTACTO
// ------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contacto {
    pub email: String,
    pub prefijo_pais: Option<String>, // FK a TblPaises
    pub telefono: String,
}

// ------------------------------------------------------------
// OTROS DATOS
// ------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtrosDatos {
    pub contratar: bool,          // Default = true
    pub observaciones: String,
    pub clas_info_1: String,
    pub clas_info_2: String,
}

//
// ============================================================
//  MODELO PRINCIPAL
// ============================================================
//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonaForm {
    pub identificacion: Identificacion,
    pub datos_personales: DatosPersonales,
    pub datos_fiscales: DatosFiscales,
    pub contacto: Contacto,
    pub otros: OtrosDatos,
}

//
// ============================================================
//  VALORES POR DEFECTO (solo estructura, sin lógica)
// ============================================================
//

impl Default for PersonaForm {
    fn default() -> Self {
        Self {
            identificacion: Identificacion {
                dni: String::new(),
                codigo: String::new(),
                tipo_documento: TipoDocumento::Dni,
                apellidos: String::new(),
                nombre: String::new(),
                naf: String::new(),
                fecha_caducidad_documento: None,
            },
            datos_personales: DatosPersonales {
                fecha_nacimiento: None,
                edad: None, // Se calculará automáticamente
                sexo: Sexo::NA,
                nacionalidad_iso3: None,
                estado_civil: EstadoCivil::Desconocido,
            },
            datos_fiscales: DatosFiscales {
                cp: String::new(),
                domicilio: String::new(),
                poblacion: String::new(),
                provincia_cod: None,
                iban: String::new(),
            },
            contacto: Contacto {
                email: String::new(),
                prefijo_pais: None,
                telefono: String::new(),
            },
            otros: OtrosDatos {
                contratar: true, // Default según tu BD
                observaciones: String::new(),
                clas_info_1: String::new(),
                clas_info_2: String::new(),
            },
        }
    }
}
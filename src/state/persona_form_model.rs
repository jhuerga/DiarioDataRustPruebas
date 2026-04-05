use serde::{Deserialize, Serialize};

//
// ============================================================
//  ENUMS
// ============================================================
//

// Tipo de documento: valores controlados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipoDocumento {
    Dni,
    Nie,
    Pasaporte,
    Otro,
}

// Sexo: valores controlados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sexo {
    Varon,
    Mujer,
    NA,
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

//
// ============================================================
//  SECCIONES DEL FORMULARIO
// ============================================================
//

// ------------------------------------------------------------
// IDENTIFICACIÓN
// ------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contacto {
    pub email: String,
    pub prefijo_pais: Option<String>, // FK a TblPaises
    pub telefono: String,
}

// ------------------------------------------------------------
// OTROS DATOS
// ------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
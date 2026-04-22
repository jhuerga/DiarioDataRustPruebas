use crate::state::persona_form_model::*;
use crate::state::validators::*;
use yew::prelude::*;
use std::rc::Rc;

//
// ============================================================
//  ESTADO PRINCIPAL DEL FORMULARIO
// ============================================================
//

#[derive(Clone, PartialEq)]
pub struct PersonaFormState {
    pub form: PersonaForm,

    pub warning_dni: Option<String>,
    pub warning_naf: Option<String>,
    pub warning_iban: Option<String>,
    pub warning_menor_edad: Option<String>,
}

impl Default for PersonaFormState {
    fn default() -> Self {
        Self {
            form: PersonaForm::default(),
            warning_dni: None,
            warning_naf: None,
            warning_iban: None,
            warning_menor_edad: None,
        }
    }
}

//
// ============================================================
//  ACCIONES DEL REDUCER (COMPLETAS Y SIN DUPLICADOS)
// ============================================================
//

pub enum PersonaFormAction {
    // IDENTIFICACIÓN
    SetDni(String),
    SetCodigo(String),
    SetTipoDocumento(TipoDocumento),
    SetApellidos(String),
    SetNombre(String),
    SetNaf(String),
    SetFechaCaducidadDocumento(Option<String>),

    // DATOS PERSONALES
    SetFechaNacimiento(Option<String>),
    SetSexo(Sexo),
    SetNacionalidad(Option<String>),
    SetEstadoCivil(EstadoCivil),

    // DATOS FISCALES
    SetCp(String),
    SetDomicilio(String),
    SetPoblacion(String),
    SetProvincia(Option<String>),
    SetIban(String),

    // CONTACTO
    SetEmail(String),
    SetPrefijoPais(Option<String>),
    SetTelefono(String),

    // OTROS DATOS
    SetContratar(bool),
    SetObservaciones(String),
    SetClasInfo1(String),
    SetClasInfo2(String),
}

//
// ============================================================
//  REDUCER
// ============================================================
//

impl Reducible for PersonaFormState {
    type Action = PersonaFormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();

        match action {

            // -------------------------
            // IDENTIFICACIÓN
            // -------------------------
            PersonaFormAction::SetDni(value) => {
                new.form.identificacion.dni = value;
                new.warning_dni = validar_dni_nie_pasaporte(
                    &new.form.identificacion.dni,
                    &new.form.identificacion.tipo_documento
                );
            }

            PersonaFormAction::SetCodigo(value) => {
                new.form.identificacion.codigo = value;
            }

            PersonaFormAction::SetTipoDocumento(value) => {
                new.form.identificacion.tipo_documento = value;
            }

            PersonaFormAction::SetApellidos(value) => {
                new.form.identificacion.apellidos = value;
            }

            PersonaFormAction::SetNombre(value) => {
                new.form.identificacion.nombre = value;
            }

            PersonaFormAction::SetNaf(value) => {
                new.form.identificacion.naf = value;
                new.warning_naf = validar_naf(&new.form.identificacion.naf);
            }

            PersonaFormAction::SetFechaCaducidadDocumento(value) => {
                new.form.identificacion.fecha_caducidad_documento = value;
            }

            // -------------------------
            // DATOS PERSONALES
            // -------------------------
            PersonaFormAction::SetFechaNacimiento(value) => {
                new.form.datos_personales.fecha_nacimiento = value;

                if let Some(fecha) = &new.form.datos_personales.fecha_nacimiento {
                    new.form.datos_personales.edad = calcular_edad(fecha);
                } else {
                    new.form.datos_personales.edad = None;
                }

                new.warning_menor_edad =
                    if let Some(edad) = new.form.datos_personales.edad {
                        if es_menor_de_edad(edad as u16) {
                            Some("La persona es menor de edad".to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
            }

            PersonaFormAction::SetSexo(value) => {
                new.form.datos_personales.sexo = value;
            }

            PersonaFormAction::SetNacionalidad(value) => {
                new.form.datos_personales.nacionalidad_iso3 = value;
            }

            PersonaFormAction::SetEstadoCivil(value) => {
                new.form.datos_personales.estado_civil = value;
            }

            // -------------------------
            // DATOS FISCALES
            // -------------------------
            PersonaFormAction::SetCp(value) => {
                new.form.datos_fiscales.cp = value;
            }

            PersonaFormAction::SetDomicilio(value) => {
                new.form.datos_fiscales.domicilio = value;
            }

            PersonaFormAction::SetPoblacion(value) => {
                new.form.datos_fiscales.poblacion = value;
            }

            PersonaFormAction::SetProvincia(value) => {
                new.form.datos_fiscales.provincia_cod = value;
            }

            PersonaFormAction::SetIban(value) => {
                new.form.datos_fiscales.iban = value;
                new.warning_iban = validar_iban(&new.form.datos_fiscales.iban);
            }

            // -------------------------
            // CONTACTO
            // -------------------------
            PersonaFormAction::SetEmail(value) => {
                new.form.contacto.email = value;
            }

            PersonaFormAction::SetPrefijoPais(value) => {
                new.form.contacto.prefijo_pais = value;
            }

            PersonaFormAction::SetTelefono(value) => {
                new.form.contacto.telefono = value;
            }

            // -------------------------
            // OTROS DATOS
            // -------------------------
            PersonaFormAction::SetContratar(value) => {
                new.form.otros.contratar = value;
            }

            PersonaFormAction::SetObservaciones(value) => {
                new.form.otros.observaciones = value;
            }

            PersonaFormAction::SetClasInfo1(value) => {
                new.form.otros.clas_info_1 = value;
            }

            PersonaFormAction::SetClasInfo2(value) => {
                new.form.otros.clas_info_2 = value;
            }
        }

        Rc::new(new)
    }
}


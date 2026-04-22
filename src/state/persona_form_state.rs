use crate::state::persona_form_model::*;
use crate::state::validators::*;
use yew::prelude::*;
use std::rc::Rc;

//
// ============================================================
//  ESTADO PRINCIPAL DEL FORMULARIO (REDUCER MODERNO)
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
//  ACCIONES DEL REDUCER
// ============================================================
//

pub enum PersonaFormAction {
    // Identificación
    SetDni(String),
    SetTipoDocumento(TipoDocumento),
    SetNaf(String),
    SetFechaCaducidadDocumento(Option<String>),

    // Datos personales
    SetFechaNacimiento(Option<String>),
    SetEstadoCivil(EstadoCivil),
    SetSexo(Sexo),
    SetNacionalidad(Option<String>),

    // Datos fiscales
    SetIban(String),
    SetCp(String),
    SetDomicilio(String),
    SetPoblacion(String),
    SetProvincia(Option<String>),

    // Contacto
    SetEmail(String),
    SetPrefijoPais(Option<String>),
    SetTelefono(String),

    // Otros datos
    SetContratar(bool),
    SetObservaciones(String),
    SetClasInfo1(String),
    SetClasInfo2(String),
}

//
// ============================================================
//  REDUCER PRINCIPAL
// ============================================================
//

impl Reducible for PersonaFormState {
    type Action = PersonaFormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();

        match action {
            //
            // IDENTIFICACIÓN
            //
            PersonaFormAction::SetDni(value) => {
                new.form.identificacion.dni = value.clone();
                new.warning_dni = validar_dni_nie_pasaporte(
                    &value,
                    &new.form.identificacion.tipo_documento,
                );
            }

            PersonaFormAction::SetTipoDocumento(value) => {
                new.form.identificacion.tipo_documento = value.clone();
                new.warning_dni = validar_dni_nie_pasaporte(
                    &new.form.identificacion.dni,
                    &value,
                );
            }

            PersonaFormAction::SetNaf(value) => {
                new.form.identificacion.naf = value.clone();
                new.warning_naf = validar_naf(&value);
            }

            PersonaFormAction::SetFechaCaducidadDocumento(value) => {
                new.form.identificacion.fecha_caducidad_documento = value;
            }

            //
            // DATOS PERSONALES
            //
            PersonaFormAction::SetFechaNacimiento(value) => {
                new.form.datos_personales.fecha_nacimiento = value.clone();

                if let Some(fecha) = value {
                    if let Some(edad) = calcular_edad(&fecha) {
                        new.form.datos_personales.edad = Some(edad);

                        new.warning_menor_edad = if es_menor_de_edad(edad) {
                            Some("La persona es menor de edad".into())
                        } else {
                            None
                        };
                    }
                }
            }

            PersonaFormAction::SetEstadoCivil(value) => {
                new.form.datos_personales.estado_civil = value;
            }

            PersonaFormAction::SetSexo(value) => {
                new.form.datos_personales.sexo = value;
            }

            PersonaFormAction::SetNacionalidad(value) => {
                new.form.datos_personales.nacionalidad_iso3 = value;
            }

            //
            // DATOS FISCALES
            //
            PersonaFormAction::SetIban(value) => {
                new.form.datos_fiscales.iban = value.clone();
                new.warning_iban = validar_iban(&value);
            }

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

            //
            // CONTACTO
            //
            PersonaFormAction::SetEmail(value) => {
                new.form.contacto.email = value;
            }

            PersonaFormAction::SetPrefijoPais(value) => {
                new.form.contacto.prefijo_pais = value;
            }

            PersonaFormAction::SetTelefono(value) => {
                new.form.contacto.telefono = value;
            }

            //
            // OTROS DATOS
            //
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

//
// ============================================================
//  HOOK PERSONALIZADO PARA EL FORMULARIO
// ============================================================
//

// Ya no se necesita, se usa use_reducer directamente en el componente.

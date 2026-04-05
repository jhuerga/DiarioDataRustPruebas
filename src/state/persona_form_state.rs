use crate::state::persona_form_model::*;
use crate::state::validators::*;
use yew::prelude::*;

#[derive(Clone)]
pub struct PersonaFormState {
    pub form: UseStateHandle<PersonaForm>,

    pub warning_dni: UseStateHandle<Option<String>>,
    pub warning_naf: UseStateHandle<Option<String>>,
    pub warning_iban: UseStateHandle<Option<String>>,
    pub warning_menor_edad: UseStateHandle<Option<String>>,
}

impl PersonaFormState {
    pub fn new(ctx: &Context<impl Component>) -> Self {
        Self {
            form: use_state(|| PersonaForm::default()),
            warning_dni: use_state(|| None),
            warning_naf: use_state(|| None),
            warning_iban: use_state(|| None),
            warning_menor_edad: use_state(|| None),
        }
    }
}

//
// ============================================================
//  SETTERS CON VALIDACIONES INTEGRADAS
// ============================================================
//

impl PersonaFormState {

    // ------------------------------------------------------------
    // IDENTIFICACIÓN
    // ------------------------------------------------------------

    pub fn set_dni(&self, value: String) {
        let mut f = (*self.form).clone();
        f.identificacion.dni = value.clone();

        // Validación DNI/NIE/PAS
        let warning = validar_dni_nie_pasaporte(&value, &f.identificacion.tipo_documento);
        self.warning_dni.set(warning);

        self.form.set(f);
    }

    pub fn set_tipo_documento(&self, value: TipoDocumento) {
        let mut f = (*self.form).clone();
        f.identificacion.tipo_documento = value.clone();

        // Revalidar DNI/NIE/PAS al cambiar el tipo
        let dni = f.identificacion.dni.clone();
        let warning = validar_dni_nie_pasaporte(&dni, &value);
        self.warning_dni.set(warning);

        self.form.set(f);
    }

    pub fn set_naf(&self, value: String) {
        let mut f = (*self.form).clone();
        f.identificacion.naf = value.clone();

        // Validación NAF
        let warning = validar_naf(&value);
        self.warning_naf.set(warning);

        self.form.set(f);
    }

    pub fn set_fecha_caducidad_documento(&self, value: Option<String>) {
        let mut f = (*self.form).clone();
        f.identificacion.fecha_caducidad_documento = value;
        self.form.set(f);
    }

    // ------------------------------------------------------------
    // DATOS PERSONALES
    // ------------------------------------------------------------

    pub fn set_fecha_nacimiento(&self, value: Option<String>) {
        let mut f = (*self.form).clone();
        f.datos_personales.fecha_nacimiento = value.clone();

        // Cálculo automático de edad
        if let Some(fecha) = value {
            if let Some(edad) = calcular_edad(&fecha) {
                f.datos_personales.edad = Some(edad);

                // Aviso menor de edad
                if es_menor_de_edad(edad) {
                    self.warning_menor_edad.set(Some("La persona es menor de edad".into()));
                } else {
                    self.warning_menor_edad.set(None);
                }
            }
        }

        self.form.set(f);
    }

    pub fn set_estado_civil(&self, value: EstadoCivil) {
        let mut f = (*self.form).clone();
        f.datos_personales.estado_civil = value;
        self.form.set(f);
    }

    pub fn set_sexo(&self, value: Sexo) {
        let mut f = (*self.form).clone();
        f.datos_personales.sexo = value;
        self.form.set(f);
    }

    pub fn set_nacionalidad(&self, value: Option<String>) {
        let mut f = (*self.form).clone();
        f.datos_personales.nacionalidad_iso3 = value;
        self.form.set(f);
    }

    // ------------------------------------------------------------
    // DATOS FISCALES
    // ------------------------------------------------------------

    pub fn set_iban(&self, value: String) {
        let mut f = (*self.form).clone();
        f.datos_fiscales.iban = value.clone();

        // Validación IBAN
        let warning = validar_iban(&value);
        self.warning_iban.set(warning);

        self.form.set(f);
    }

    pub fn set_cp(&self, value: String) {
        let mut f = (*self.form).clone();
        f.datos_fiscales.cp = value;
        self.form.set(f);
    }

    pub fn set_domicilio(&self, value: String) {
        let mut f = (*self.form).clone();
        f.datos_fiscales.domicilio = value;
        self.form.set(f);
    }

    pub fn set_poblacion(&self, value: String) {
        let mut f = (*self.form).clone();
        f.datos_fiscales.poblacion = value;
        self.form.set(f);
    }

    pub fn set_provincia(&self, value: Option<String>) {
        let mut f = (*self.form).clone();
        f.datos_fiscales.provincia_cod = value;
        self.form.set(f);
    }

    // ------------------------------------------------------------
    // CONTACTO
    // ------------------------------------------------------------

    pub fn set_email(&self, value: String) {
        let mut f = (*self.form).clone();
        f.contacto.email = value;
        self.form.set(f);
    }

    pub fn set_prefijo_pais(&self, value: Option<String>) {
        let mut f = (*self.form).clone();
        f.contacto.prefijo_pais = value;
        self.form.set(f);
    }

    pub fn set_telefono(&self, value: String) {
        let mut f = (*self.form).clone();
        f.contacto.telefono = value;
        self.form.set(f);
    }

    // ------------------------------------------------------------
    // OTROS DATOS
    // ------------------------------------------------------------

    pub fn set_contratar(&self, value: bool) {
        let mut f = (*self.form).clone();
        f.otros.contratar = value;
        self.form.set(f);
    }

    pub fn set_observaciones(&self, value: String) {
        let mut f = (*self.form).clone();
        f.otros.observaciones = value;
        self.form.set(f);
    }

    pub fn set_clas_info_1(&self, value: String) {
        let mut f = (*self.form).clone();
        f.otros.clas_info_1 = value;
        self.form.set(f);
    }

    pub fn set_clas_info_2(&self, value: String) {
        let mut f = (*self.form).clone();
        f.otros.clas_info_2 = value;
        self.form.set(f);
    }
}
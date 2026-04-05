use chrono::{Datelike, NaiveDate};
use crate::state::persona_form_model::TipoDocumento;

//
// ============================================================
//  VALIDACIÓN DNI / NIE / PASAPORTE
// ============================================================
//

pub fn validar_dni_nie_pasaporte(valor: &str, tipo: &TipoDocumento) -> Option<String> {
    let v = valor.trim().to_uppercase();

    match tipo {
        TipoDocumento::Dni => validar_dni(&v),
        TipoDocumento::Nie => validar_nie(&v),
        TipoDocumento::Pasaporte => validar_pasaporte(&v),
        TipoDocumento::Otro => None, // No validamos
    }
}

//
// ---------------------- DNI ----------------------
//

fn validar_dni(dni: &str) -> Option<String> {
    if dni.len() != 9 {
        return Some("El DNI debe tener 8 dígitos y 1 letra".into());
    }

    let (num_str, letra_str) = dni.split_at(8);

    if !num_str.chars().all(|c| c.is_ascii_digit()) {
        return Some("Los primeros 8 caracteres del DNI deben ser números".into());
    }

    let letra = letra_str.chars().next().unwrap();
    if !letra.is_ascii_alphabetic() {
        return Some("El último carácter del DNI debe ser una letra".into());
    }

    let letras = "TRWAGMYFPDXBNJZSQVHLCKE";
    let num: usize = num_str.parse().unwrap();
    let letra_correcta = letras.chars().nth(num % 23).unwrap();

    if letra != letra_correcta {
        return Some(format!(
            "La letra del DNI no es correcta. Debería ser {}",
            letra_correcta
        ));
    }

    None
}

//
// ---------------------- NIE ----------------------
//

fn validar_nie(nie: &str) -> Option<String> {
    if nie.len() != 9 {
        return Some("El NIE debe tener 1 letra inicial, 7 dígitos y 1 letra final".into());
    }

    let primera = nie.chars().next().unwrap();
    let ultima = nie.chars().last().unwrap();

    if !"XYZ".contains(primera) {
        return Some("El NIE debe comenzar por X, Y o Z".into());
    }

    let cuerpo = &nie[1..8];
    if !cuerpo.chars().all(|c| c.is_ascii_digit()) {
        return Some("Los caracteres centrales del NIE deben ser números".into());
    }

    let letras = "TRWAGMYFPDXBNJZSQVHLCKE";

    let sustitucion = match primera {
        'X' => "0",
        'Y' => "1",
        'Z' => "2",
        _ => unreachable!(),
    };

    let numero_completo = format!("{}{}", sustitucion, cuerpo);
    let num: usize = numero_completo.parse().unwrap();
    let letra_correcta = letras.chars().nth(num % 23).unwrap();

    if ultima != letra_correcta {
        return Some(format!(
            "La letra del NIE no es correcta. Debería ser {}",
            letra_correcta
        ));
    }

    None
}

//
// ---------------------- PASAPORTE ----------------------
//

fn validar_pasaporte(p: &str) -> Option<String> {
    if p.len() < 3 {
        return Some("El pasaporte debe tener al menos 3 caracteres".into());
    }

    if !p.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Some("El pasaporte solo puede contener letras y números".into());
    }

    None
}

//
// ============================================================
//  VALIDACIÓN NAF (SEGURIDAD SOCIAL)
// ============================================================
//

pub fn validar_naf(naf: &str) -> Option<String> {
    let v = naf.trim();

    if v.len() != 12 {
        return Some("El NAF debe tener exactamente 12 dígitos".into());
    }

    if !v.chars().all(|c| c.is_ascii_digit()) {
        return Some("El NAF solo puede contener números".into());
    }

    let (base, dc_str) = v.split_at(10);
    let base_num: u128 = base.parse().unwrap();
    let dc: u8 = dc_str.parse().unwrap();

    let calculado = (base_num % 97) as u8;

    if calculado != dc {
        return Some(format!(
            "Dígitos de control incorrectos. Deberían ser {:02}",
            calculado
        ));
    }

    None
}

//
// ============================================================
//  VALIDACIÓN IBAN (ESPAÑA)
// ============================================================
//

pub fn validar_iban(iban: &str) -> Option<String> {
    let v = iban.trim().replace(" ", "").to_uppercase();

    if !v.starts_with("ES") {
        return Some("El IBAN debe comenzar por ES".into());
    }

    if v.len() != 24 {
        return Some("El IBAN español debe tener 24 caracteres".into());
    }

    // Reordenar: mover ESXX al final
    let reordenado = format!("{}{}", &v[4..], &v[..4]);

    // Convertir letras a números
    let mut numerico = String::new();
    for c in reordenado.chars() {
        if c.is_ascii_digit() {
            numerico.push(c);
        } else if c.is_ascii_alphabetic() {
            numerico.push_str(&(c as u32 - 55).to_string());
        } else {
            return Some("El IBAN contiene caracteres no válidos".into());
        }
    }

    // Validación mod 97
    let mut resto = 0u128;
    for ch in numerico.chars() {
        resto = (resto * 10 + ch.to_digit(10).unwrap() as u128) % 97;
    }

    if resto != 1 {
        return Some("El IBAN no supera la validación de control (MOD 97)".into());
    }

    None
}

//
// ============================================================
//  CÁLCULO DE EDAD
// ============================================================
//

pub fn calcular_edad(fecha: &str) -> Option<u16> {
    let fecha_nac = NaiveDate::parse_from_str(fecha, "%Y-%m-%d").ok()?;
    let hoy = chrono::Local::now().date_naive();

    let mut edad = hoy.year() - fecha_nac.year();

    if hoy.ordinal() < fecha_nac.ordinal() {
        edad -= 1;
    }

    Some(edad as u16)
}

pub fn es_menor_de_edad(edad: u16) -> bool {
    edad < 18
}
use crate::modelo::Esquema;
use chrono::{Datelike, Days, NaiveDate};

pub fn parsear_fecha(text: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").map_err(|_| format!("Fecha inválida: {}", text))
}

pub fn es_fecha_quincenal(fecha: NaiveDate) -> bool {
    let es_ultimo = fecha.succ_opt().is_some_and(|sig| sig.day() == 1);
    fecha.day() == 15 || es_ultimo
}

fn ultimo_dia_del_mes(anio: i32, mes: u32) -> Option<NaiveDate> {
    (28..=31)
        .rev()
        .find_map(|dia| NaiveDate::from_ymd_opt(anio, mes, dia))
}

fn siguiente_quincena(f: NaiveDate) -> Option<NaiveDate> {
    if f.day() == 15 {
        ultimo_dia_del_mes(f.year(), f.month())
    } else {
        f.succ_opt()?.with_day(15)
    }
}

fn fecha_quincenal(inicio: NaiveDate, i: u32) -> Result<NaiveDate, String> {
    if !es_fecha_quincenal(inicio) {
        return Err(String::from(
            "En quincenal el primer pago debe ser el dia 15 o el ultimo dia del mes",
        ));
    }
    let mut fecha = inicio;
    for _ in 0..i {
        fecha = siguiente_quincena(fecha).ok_or_else(|| String::from("Fecha fuera de rango"))?;
    }
    Ok(fecha)
}

pub fn fecha_de_cuota(inicio: NaiveDate, esquema: Esquema, i: u32) -> Result<NaiveDate, String> {
    match esquema {
        Esquema::Semanal => inicio
            .checked_add_days(Days::new(i as u64 * 7))
            .ok_or_else(|| String::from("Fecha fuera de rango")),
        Esquema::Quincenal => fecha_quincenal(inicio, i),
    }
}

pub fn crear_prestamo(
    id: u32,
    deudor: &str,
    fecha_primer_pago: &str,
    monto_prestado: i64,
    monto_pago: i64,
    monto_devuelve: i64,
    esquema: Esquema,
) -> Result<crate::modelo::Prestamo, String> {
    let fecha = parsear_fecha(fecha_primer_pago)?;

    if monto_prestado <= 0 {
        return Err(String::from("El monto prestado debe ser mayor a 0"));
    }
    if monto_devuelve < monto_prestado {
        return Err(String::from(
            "El monto a devolver no puede ser menor al prestado",
        ));
    }
    if monto_pago <= 0 {
        return Err(String::from("El monto de cada pago debe ser mayor a 0"));
    }
    if deudor.trim().len() < 2 {
        return Err(String::from("El deudor debe tener al menos 2 caracteres"));
    }
    if let Esquema::Quincenal = esquema {
        if !es_fecha_quincenal(fecha) {
            return Err(String::from(
                "En quincenal el primer pago debe ser el dia 15 o el ultimo dia del mes",
            ));
        }
    }

    Ok(crate::modelo::Prestamo {
        id,
        deudor: deudor.trim().to_string(),
        fecha_primer_pago: fecha,
        monto_prestado,
        monto_devuelve,
        monto_pago,
        esquema,
        pagos: Vec::new(),
    })
}

pub fn generar_tabla(p: &crate::modelo::Prestamo) -> Result<Vec<crate::modelo::Cuota>, String> {
    if p.monto_pago <= 0 {
        return Err(String::from("El monto de cada pago debe ser mayor a 0"));
    }

    let mut tabla = Vec::new();
    let mut restante = p.monto_devuelve;
    let mut numero: u32 = 0;

    while restante > 0 {
        let fecha = fecha_de_cuota(p.fecha_primer_pago, p.esquema, numero)?;
        let monto = restante.min(p.monto_pago);
        restante -= monto;
        numero += 1;
        tabla.push(crate::modelo::Cuota {
            numero,
            fecha,
            monto,
        });
    }

    Ok(tabla)
}

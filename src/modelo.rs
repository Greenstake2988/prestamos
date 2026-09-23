use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Esquema {
    Semanal,
    Quincenal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cuota {
    pub numero: u32,
    pub fecha: NaiveDate,
    pub monto: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    pub fecha: NaiveDate,
    pub monto: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prestamo {
    pub id: u32,
    pub deudor: String,
    pub fecha_primer_pago: NaiveDate,
    pub monto_prestado: i64,
    pub monto_devuelve: i64,
    pub monto_pago: i64,
    pub esquema: Esquema,
    pub pagos: Vec<Pago>,
}

pub fn formatear(centavos: i64) -> String {
    format!("{}.{:02}", centavos / 100, centavos % 100)
}

pub fn total_pagado(p: &Prestamo) -> i64 {
    p.pagos.iter().map(|pago| pago.monto).sum()
}

pub fn saldo(p: &Prestamo) -> i64 {
    p.monto_devuelve - total_pagado(p)
}

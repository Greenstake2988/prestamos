use chrono::{Datelike, Days, NaiveDate};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook, XlsxError};

#[derive(Debug, Clone, Copy)]
enum Esquema {
    Semanal,
    Quincenal,
}

#[derive(Debug, Clone)]
struct Cuota {
    numero: u32,
    fecha: NaiveDate,
    monto: i64, // centavos
}

#[derive(Debug, Clone)]
struct Pago {
    fecha: NaiveDate,
    monto: i64,
}

#[derive(Debug, Clone)]
struct Prestamo {
    id: u32,
    fecha_primer_pago: NaiveDate,
    monto_pago: i64,
    deudor: String,
    monto_prestado: i64,
    monto_devuelve: i64,
    esquema: Esquema,
    pagos: Vec<Pago>,
}

fn total_pagado(p: &Prestamo) -> i64 {
    p.pagos.iter().map(|pago| pago.monto).sum()
}

fn es_fecha_quincenal(fecha: NaiveDate) -> bool {
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
        f.succ_opt()?.with_day(15) // dia sigueinte = dia 1 del mes que sigue -> su 15
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

fn fecha_de_cuota(inicio: NaiveDate, esquema: Esquema, i: u32) -> Result<NaiveDate, String> {
    match esquema {
        Esquema::Semanal => inicio
            .checked_add_days(Days::new(i as u64 * 7))
            .ok_or_else(|| String::from("Fecha fuera de rango")),
        Esquema::Quincenal => fecha_quincenal(inicio, i),
    }
}

fn crear_prestamo(
    id: u32,
    deudor: &str,
    fecha_primer_pago: &str,
    monto_prestado: i64,
    monto_pago: i64,
    monto_devuelve: i64,
    esquema: Esquema,
) -> Result<Prestamo, String> {
    let fecha = parsear_fecha(fecha_primer_pago)?; // si fall, sale de la funcion

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
    Ok(Prestamo {
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

fn generar_tabla(p: &Prestamo) -> Result<Vec<Cuota>, String> {
    if p.monto_pago <= 0 {
        return Err(String::from("El monto de cada pago debe ser mayor a 0"));
    }

    let mut tabla = Vec::new();
    let mut restante = p.monto_devuelve;
    let mut numero: u32 = 0;

    while restante > 0 {
        let fecha = fecha_de_cuota(p.fecha_primer_pago, p.esquema, numero)?;
        let monto = restante.min(p.monto_pago); // la ultima cuota toma lo que quede
        restante -= monto;
        numero += 1;
        tabla.push(Cuota {
            numero,
            fecha,
            monto,
        });
    }

    Ok(tabla)
}

fn formatear(centavos: i64) -> String {
    format!("{}.{:02}", centavos / 100, centavos % 100)
}

/// Imprime todo: datos del préstamo y cada cuota de la tabla.
fn imprimir_tabla(p: &Prestamo) {
    match generar_tabla(p) {
        Ok(tabla) => {
            println!("Préstamo #{} - {}", p.id, p.deudor);
            println!("Prestado:       {}", formatear(p.monto_prestado));
            println!("A devolver:     {}", formatear(p.monto_devuelve));
            println!(
                "Ganancia:       {}",
                formatear(p.monto_devuelve - p.monto_prestado)
            );
            println!("Pago por cuota: {}", formatear(p.monto_pago));
            println!("Esquema:        {:?}", p.esquema);
            println!("Cuotas:         {}", tabla.len());
            println!("-----------------------------");
            for c in &tabla {
                println!("{:>2} | {} | {:>10}", c.numero, c.fecha, formatear(c.monto));
            }
            let total: i64 = tabla.iter().map(|c| c.monto).sum();
            println!("-----------------------------");
            println!("Suma de la tabla: {}", formatear(total));
        }
        Err(e) => println!("Error: {}", e),
    }
}
fn nombre_esquema(e: Esquema) -> &'static str {
    match e {
        Esquema::Semanal => "Semanal",
        Esquema::Quincenal => "Quincenal",
    }
}

fn exportar_excel(p: &Prestamo, ruta: &str) -> Result<(), String> {
    let tabla = generar_tabla(p)?;
    escribir_excel(p, &tabla, ruta).map_err(|e| format!("No se pudo crear el Excel: {}", e))
}

fn escribir_excel(p: &Prestamo, tabla: &[Cuota], ruta: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let hoja = workbook.add_worksheet();
    hoja.set_name("Tabla de pagos")?;

    // Formatos
    let encabezado = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xD9E1F2))
        .set_border(FormatBorder::Thin);
    let celda = Format::new().set_border(FormatBorder::Thin);
    let fecha_fmt = Format::new()
        .set_num_format("dd-mmm-yy")
        .set_border(FormatBorder::Thin);
    let dinero_fmt = Format::new()
        .set_num_format("$#,##0.00")
        .set_border(FormatBorder::Thin);
    let total_fmt = Format::new()
        .set_bold()
        .set_num_format("$#,##0.00")
        .set_border(FormatBorder::Thin);
    let etiqueta = Format::new().set_bold();
    let info_fecha = Format::new().set_num_format("dd-mmm-yy");
    let info_dinero = Format::new().set_num_format("$#,##0.00");

    let pesos = |centavos: i64| centavos as f64 / 100.0;

    // ---------- Tabla (columnas A-C) ----------
    hoja.write_with_format(0, 0, "N° de Pago", &encabezado)?;
    hoja.write_with_format(0, 1, "Fecha de Pago", &encabezado)?;
    hoja.write_with_format(0, 2, "Monto", &encabezado)?;

    for (i, c) in tabla.iter().enumerate() {
        let fila = (i + 1) as u32;
        hoja.write_with_format(fila, 0, c.numero, &celda)?;
        hoja.write_with_format(fila, 1, &c.fecha, &fecha_fmt)?;
        hoja.write_with_format(fila, 2, pesos(c.monto), &dinero_fmt)?;
    }

    // Total con fórmula de Excel
    let fila_total = (tabla.len() + 1) as u32;
    let formula = format!("=SUM(C2:C{})", tabla.len() + 1);
    hoja.write_with_format(fila_total, 0, "TOTAL", &total_fmt)?;
    hoja.write_with_format(fila_total, 1, "", &total_fmt)?;
    hoja.write_formula_with_format(fila_total, 2, formula.as_str(), &total_fmt)?;

    // ---------- Datos del préstamo (columnas E-F) ----------
    hoja.write_with_format(0, 4, "Nombre:", &etiqueta)?;
    hoja.write(0, 5, p.deudor.as_str())?;

    hoja.write_with_format(1, 4, "Monto Prestado:", &etiqueta)?;
    hoja.write_with_format(1, 5, pesos(p.monto_prestado), &info_dinero)?;

    hoja.write_with_format(2, 4, "Esquema de Pago:", &etiqueta)?;
    hoja.write(2, 5, nombre_esquema(p.esquema))?;

    hoja.write_with_format(3, 4, "Fecha de Inicio:", &etiqueta)?;
    hoja.write_with_format(3, 5, &p.fecha_primer_pago, &info_fecha)?;

    if let Some(ultima) = tabla.last() {
        hoja.write_with_format(4, 4, "Fecha de Fin:", &etiqueta)?;
        hoja.write_with_format(4, 5, &ultima.fecha, &info_fecha)?;
    }

    hoja.write_with_format(5, 4, "Número de Pagos:", &etiqueta)?;
    hoja.write(5, 5, tabla.len() as u32)?;

    hoja.write_with_format(6, 4, "Monto por Pago:", &etiqueta)?;
    hoja.write_with_format(6, 5, pesos(p.monto_pago), &info_dinero)?;

    // Anchos de columna
    hoja.set_column_width(0, 12)?;
    hoja.set_column_width(1, 16)?;
    hoja.set_column_width(2, 14)?;
    hoja.set_column_width(3, 4)?; // columna separadora
    hoja.set_column_width(4, 18)?;
    hoja.set_column_width(5, 16)?;

    workbook.save(ruta)?;
    Ok(())
}

fn parsear_fecha(text: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").map_err(|_| format!("Fecha inválida: {}", text))
}
fn main() {
    match crear_prestamo(
        1,
        "Leidy",
        "2026-12-15",
        200_000,
        40_000,
        400_000,
        Esquema::Semanal,
    ) {
        Ok(p) => {
            imprimir_tabla(&p);
            match exportar_excel(&p, "prestamo_1.xlsx") {
                Ok(()) => println!("Excel guardaddo: prestamo_1.xlsx"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(mensaje) => println!("Error: {}", mensaje),
    }

    println!();
    // Préstamo 2: quincenal (el primer pago debe ser día 15 o último día del mes)
    match crear_prestamo(
        2,
        "Carlos",
        "2026-12-31",
        100_000, // prestado: 1,000.00
        30_000,  // pago por cuota: 300.00
        120_000, // a devolver: 1,200.00
        Esquema::Quincenal,
    ) {
        Ok(p) => imprimir_tabla(&p),
        Err(mensaje) => println!("Error: {}", mensaje),
    }
}

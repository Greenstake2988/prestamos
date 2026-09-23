use crate::modelo::{Cuota, Prestamo};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook, XlsxError};

fn nombre_esquema(e: crate::modelo::Esquema) -> &'static str {
    match e {
        crate::modelo::Esquema::Semanal => "Semanal",
        crate::modelo::Esquema::Quincenal => "Quincenal",
    }
}

pub fn exportar_excel(p: &Prestamo, ruta: &str) -> Result<(), String> {
    let tabla = crate::fechas::generar_tabla(p)?;
    escribir_excel(p, &tabla, ruta).map_err(|e| format!("No se pudo crear el Excel: {}", e))
}

fn escribir_excel(p: &Prestamo, tabla: &[Cuota], ruta: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let hoja = workbook.add_worksheet();
    hoja.set_name("Tabla de pagos")?;

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

    hoja.write_with_format(0, 0, "N° de Pago", &encabezado)?;
    hoja.write_with_format(0, 1, "Fecha de Pago", &encabezado)?;
    hoja.write_with_format(0, 2, "Monto", &encabezado)?;

    for (i, c) in tabla.iter().enumerate() {
        let fila = (i + 1) as u32;
        hoja.write_with_format(fila, 0, c.numero, &celda)?;
        hoja.write_with_format(fila, 1, &c.fecha, &fecha_fmt)?;
        hoja.write_with_format(fila, 2, pesos(c.monto), &dinero_fmt)?;
    }

    let fila_total = (tabla.len() + 1) as u32;
    let formula = format!("=SUM(C2:C{})", tabla.len() + 1);
    hoja.write_with_format(fila_total, 0, "TOTAL", &total_fmt)?;
    hoja.write_with_format(fila_total, 1, "", &total_fmt)?;
    hoja.write_formula_with_format(fila_total, 2, formula.as_str(), &total_fmt)?;

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

    hoja.set_column_width(0, 12)?;
    hoja.set_column_width(1, 16)?;
    hoja.set_column_width(2, 14)?;
    hoja.set_column_width(3, 4)?;
    hoja.set_column_width(4, 18)?;
    hoja.set_column_width(5, 16)?;

    workbook.save(ruta)?;
    Ok(())
}

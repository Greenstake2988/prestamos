use crate::EstadoApp;
use crate::fechas::crear_prestamo;
use crate::modelo::Cuota;
use crate::modelo::{Esquema, Prestamo};
use axum::extract::Path;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;

fn pesos_a_centavos(pesos: f64) -> i64 {
    (pesos * 100.0).round() as i64
}

#[derive(Debug, Deserialize)]
pub struct NuevoPrestamoJson {
    deudor: String,
    fecha_primer_pago: String,
    monto_prestado: f64,
    monto_pago: f64,
    monto_a_devolver: f64,
    esquema: Esquema,
}

pub async fn crear_prestamo_handler(
    State(estado): State<EstadoApp>,
    Json(datos): Json<NuevoPrestamoJson>,
) -> Result<Json<Prestamo>, (StatusCode, String)> {
    let mut prestamos = estado.prestamos.lock().unwrap();
    let siguiente_id = prestamos.iter().len() as u32 + 1;

    let prestamo = crear_prestamo(
        siguiente_id,
        &datos.deudor,
        &datos.fecha_primer_pago,
        pesos_a_centavos(datos.monto_prestado),
        pesos_a_centavos(datos.monto_pago),
        pesos_a_centavos(datos.monto_a_devolver),
        datos.esquema,
    )
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    prestamos.push(prestamo.clone());
    Ok(Json(prestamo))
}

pub async fn listar_prestamos_handler(State(estado): State<EstadoApp>) -> Json<Vec<Prestamo>> {
    let prestamos = estado.prestamos.lock().unwrap();
    Json(prestamos.clone())
}

pub async fn tabla_prestamo_handler(
    State(estado): State<EstadoApp>,
    Path(id): Path<u32>,
) -> Result<Json<Vec<Cuota>>, (StatusCode, String)> {
    let prestamos = estado.prestamos.lock().unwrap();

    let prestamo = prestamos.iter().find(|p| p.id == id).ok_or((
        StatusCode::NOT_FOUND,
        format!("No existe el préstamo {}", id),
    ))?;

    let tabla = crate::fechas::generar_tabla(prestamo).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(tabla))
}

pub async fn excel_prestamo_handler(
    State(estado): State<EstadoApp>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let prestamo = {
        let prestamos = estado.prestamos.lock().unwrap();
        prestamos.iter().find(|p| p.id == id).cloned().ok_or((
            StatusCode::NOT_FOUND,
            format!("No existe el préstamo {}", id),
        ))?
    };

    let ruta_temporal = format!("/tmp/prestamo_{}.xlsx", id);
    crate::excel::exportar_excel(&prestamo, &ruta_temporal)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let bytes = std::fs::read(&ruta_temporal)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let nombre_archivo = format!("prestamo_{}_{}.xlsx", id, prestamo.deudor);

    Ok((
        [
            (
                header::CONTENT_TYPE,
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            ),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", nombre_archivo),
            ),
        ],
        bytes,
    ))
}

json.extract! prestamo, :id, :persona_id, :monto_prestado, :monto_a_devolver, :monto_pago, :fecha_primer_pago, :frecuencia, :created_at, :updated_at
json.url prestamo_url(prestamo, format: :json)

class RespaldoXlsx
  # Sin comillas dobles: dentro del XML del archivo dañaban los estilos.
  MONEDA = '$#,##0.00\ \M\X\N'.freeze
  FECHA  = "dd/mm/yyyy".freeze

  def initialize
    @prestamos = Prestamo.includes(:persona).order(:fecha_primer_pago)
  end

  def render
    package = Axlsx::Package.new
    libro = package.workbook

    @titulo         = libro.styles.add_style(b: true, sz: 16)
    @negrita        = libro.styles.add_style(b: true)
    @encabezado     = libro.styles.add_style(b: true, bg_color: "DDDDDD",
                                             border: { style: :thin, color: "999999" })
    @moneda         = libro.styles.add_style(format_code: MONEDA)
    @moneda_negrita = libro.styles.add_style(format_code: MONEDA, b: true)
    @fecha          = libro.styles.add_style(format_code: FECHA, alignment: { horizontal: :left })

    @usados = []
    hoja_resumen(libro)
    @prestamos.group_by(&:persona).each { |persona, lista| hoja_persona(libro, persona, lista) }

    package.to_stream.read
  end

  private

  # Pestaña con todos los préstamos y totales (información tuya, incluye ganancia)
  def hoja_resumen(libro)
    libro.add_worksheet(name: "Resumen") do |hoja|
      @usados << "Resumen"

      hoja.add_row ["Resumen de préstamos"], style: @titulo
      hoja.add_row
      hoja.add_row ["Persona", "Prestado", "A devolver", "Ganancia", "Monto de cada pago",
                    "Núm. de pagos", "Frecuencia", "Primer pago"], style: @encabezado

      @prestamos.each do |p|
        hoja.add_row [p.persona.nombre, p.monto_prestado.to_f, p.monto_a_devolver.to_f,
                      p.ganancia.to_f, p.monto_pago.to_f, p.numero_de_pagos,
                      p.frecuencia.capitalize, p.fecha_primer_pago],
                     style: [nil, @moneda, @moneda, @moneda, @moneda, nil, nil, @fecha],
                     types: [:string, :float, :float, :float, :float, :integer, :string, :date]
      end

      if @prestamos.any?
        hoja.add_row ["Total",
                      @prestamos.sum(&:monto_prestado).to_f,
                      @prestamos.sum(&:monto_a_devolver).to_f,
                      @prestamos.sum(&:ganancia).to_f],
                     style: [@negrita, @moneda_negrita, @moneda_negrita, @moneda_negrita],
                     types: [:string, :float, :float, :float]
      end

      hoja.column_widths 24, 18, 18, 18, 20, 14, 14, 14
    end
  end

  # Una pestaña por persona, mismo formato que el PDF (sin ganancia)
  def hoja_persona(libro, persona, prestamos)
    libro.add_worksheet(name: nombre_hoja(persona.nombre)) do |hoja|
      hoja.add_row ["Calendario de pagos"], style: @titulo
      hoja.add_row ["Nombre:", persona.nombre], style: [@negrita, nil]
      hoja.add_row

      prestamos.each do |p|
        hoja.add_row ["Monto prestado:", p.monto_prestado.to_f],
                     style: [@negrita, @moneda], types: [:string, :float]
        hoja.add_row ["Total a devolver:", p.monto_a_devolver.to_f],
                     style: [@negrita, @moneda], types: [:string, :float]
        hoja.add_row ["Número de pagos:", "#{p.numero_de_pagos} (#{p.frecuencia})"],
                     style: [@negrita, nil]
        hoja.add_row
        hoja.add_row ["#", "Fecha", "Monto"], style: @encabezado

        p.calendario.each do |pago|
          hoja.add_row [pago[:numero], pago[:fecha], pago[:monto].to_f],
                       style: [nil, @fecha, @moneda], types: [:integer, :date, :float]
        end

        hoja.add_row
        hoja.add_row
      end

      hoja.column_widths 22, 22, 20
    end
  end

  # Excel limita el nombre a 31 caracteres, sin []:*?/\ y sin repetirse
  def nombre_hoja(nombre)
    base = nombre.to_s.gsub(%r{[\[\]:*?/\\]}, "").strip.first(28)
    base = "Persona" if base.blank?
    candidato = base
    n = 2
    while @usados.map(&:downcase).include?(candidato.downcase)
      candidato = "#{base} #{n}"
      n += 1
    end
    @usados << candidato
    candidato
  end
end

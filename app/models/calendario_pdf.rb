class CalendarioPdf
  include ActionView::Helpers::NumberHelper

  def initialize(prestamo)
    @prestamo = prestamo
  end

  def render
    pdf = Prawn::Document.new(page_size: "A4", margin: 40)

    pdf.text "Calendario de pagos", size: 22, style: :bold
    pdf.move_down 10
    pdf.text "Nombre: #{@prestamo.persona.nombre}"
    pdf.text "Monto prestado: #{number_to_currency(@prestamo.monto_prestado)}"
    pdf.text "Total a devolver: #{number_to_currency(@prestamo.monto_a_devolver)}"
    pdf.text "Número de pagos: #{@prestamo.numero_de_pagos} (#{frecuencia})"
    pdf.move_down 15

    filas = [["#", "Fecha", "Monto"]]
    @prestamo.calendario.each do |pago|
      filas << [pago[:numero].to_s, I18n.l(pago[:fecha], format: :long), number_to_currency(pago[:monto])]
    end

    pdf.table(filas, header: true, width: pdf.bounds.width, cell_style: { padding: 6 }) do
      row(0).font_style = :bold
      row(0).background_color = "DDDDDD"
      columns(2).align = :right
    end

    pdf.render
  end

  private

  def frecuencia
    @prestamo.frecuencia == "quincenal" ? "quincenal, días 15 y fin de mes" : "semanal"
  end
end

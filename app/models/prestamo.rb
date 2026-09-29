class Prestamo < ApplicationRecord
  FRECUENCIAS = %w[semanal quincenal].freeze

  belongs_to :persona

  validates :monto_prestado, :monto_a_devolver, :monto_pago,
            numericality: { greater_than: 0 }
  validates :fecha_primer_pago, presence: true
  validates :frecuencia, inclusion: { in: FRECUENCIAS }
  validate :numero_de_pagos_razonable

  def ganancia
    monto_a_devolver - monto_prestado
  end

  def numero_de_pagos
    (monto_a_devolver / monto_pago).ceil
  end

  def fechas_de_pago
    case frecuencia
    when "semanal"
      (0...numero_de_pagos).map { |i| fecha_primer_pago + (i * 7).days }
    when "quincenal"
      fechas_quincenales
    end
  end

  def calendario
    fechas_de_pago.each_with_index.map do |fecha, i|
      {
        numero: i + 1,
        fecha: fecha,
        monto: [monto_pago, monto_a_devolver - (monto_pago * i)].min
      }
    end
  end

  def self.proximos_pagos(desde: Date.current, hasta: 14.days.from_now.to_date)
    includes(:persona).flat_map do |prestamo|
      prestamo.calendario.select { |p| p[:fecha].between?(desde, hasta) }
                         .map { |p| p.merge(persona: prestamo.persona, prestamo: prestamo) }
    end.sort_by { |p| p[:fecha] }
  end

  def pendiente_estimado(hoy: Date.current)
      calendario.select { |p| p[:fecha] > hoy }.sum { |p| p[:monto] }
    end
  private

  def fechas_quincenales
    fechas = []
    dia = fecha_primer_pago
    while fechas.size < numero_de_pagos
      fechas << dia if dia.day == 15 || dia == dia.end_of_month
      dia += 1.day
    end
    fechas
  end

  def numero_de_pagos_razonable
    return if monto_pago.blank? || monto_a_devolver.blank? || monto_pago <= 0
    if numero_de_pagos > 104
      errors.add(:monto_pago, "genera demasiados pagos (máximo 104)")
    end
  end
end

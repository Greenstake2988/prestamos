class InicioController < ApplicationController
  def index
    @total_prestado = Prestamo.sum(:monto_prestado)
    @total_a_devolver = Prestamo.sum(:monto_a_devolver)
    @ganancia = @total_a_devolver - @total_prestado
    @proximos_pagos = Prestamo.proximos_pagos
  end
end

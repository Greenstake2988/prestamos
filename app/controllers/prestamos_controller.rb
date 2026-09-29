require "csv"

class PrestamosController < ApplicationController
  before_action :set_prestamo, only: %i[ show edit update destroy pdf ]

  # GET /prestamos or /prestamos.json
  def index
    @prestamos = Prestamo.all
  end

  # GET /prestamos/1 or /prestamos/1.json
  def show
  end

  # GET /prestamos/new
  def new
    @prestamo = Prestamo.new(persona_id: params[:persona_id])
  end

  # GET /prestamos/1/edit
  def edit
  end

  # POST /prestamos or /prestamos.json
  def create
    @prestamo = Prestamo.new(prestamo_params)

    respond_to do |format|
      if @prestamo.save
        format.html { redirect_to @prestamo, notice: "Préstamo creado." }
        format.json { render :show, status: :created, location: @prestamo }
      else
        format.html { render :new, status: :unprocessable_content }
        format.json { render json: @prestamo.errors, status: :unprocessable_content }
      end
    end
  end

  # PATCH/PUT /prestamos/1 or /prestamos/1.json
  def update
    respond_to do |format|
      if @prestamo.update(prestamo_params)
        format.html { redirect_to @prestamo, notice: "Préstamo actualizado.", status: :see_other }
        format.json { render :show, status: :ok, location: @prestamo }
      else
        format.html { render :edit, status: :unprocessable_content }
        format.json { render json: @prestamo.errors, status: :unprocessable_content }
      end
    end
  end

  # DELETE /prestamos/1 or /prestamos/1.json
  def destroy
    @prestamo.destroy!

    respond_to do |format|
      format.html { redirect_to prestamos_path, notice: "Préstamo eliminado.", status: :see_other }
      format.json { head :no_content }
    end
  end

  def pdf
    contenido = CalendarioPdf.new(@prestamo).render
    send_data contenido,
              filename: "pagos-#{@prestamo.persona.nombre.parameterize}.pdf",
              type: "application/pdf",
              disposition: "inline"
  end



  def exportar
    csv = CSV.generate do |fila|
      fila << ["Persona", "Monto prestado", "Monto a devolver", "Ganancia",
               "Monto de cada pago", "Número de pagos", "Frecuencia", "Fecha primer pago"]

      Prestamo.includes(:persona).order(:fecha_primer_pago).each do |p|
        fila << [p.persona.nombre, p.monto_prestado, p.monto_a_devolver, p.ganancia,
                 p.monto_pago, p.numero_de_pagos, p.frecuencia, p.fecha_primer_pago]
      end
    end

    send_data "\uFEFF" + csv,
              filename: "prestamos-#{Date.current}.csv",
              type: "text/csv; charset=utf-8"
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_prestamo
      @prestamo = Prestamo.find(params.expect(:id))
    end

    # Only allow a list of trusted parameters through.
    def prestamo_params
      params.expect(prestamo: [ :persona_id, :monto_prestado, :monto_a_devolver, :monto_pago, :fecha_primer_pago, :frecuencia ])    end
end

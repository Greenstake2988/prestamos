class PrestamosController < ApplicationController
  before_action :set_prestamo, only: %i[ show edit update destroy ]

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
        format.html { redirect_to @prestamo, notice: "Prestamo was successfully created." }
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
        format.html { redirect_to @prestamo, notice: "Prestamo was successfully updated.", status: :see_other }
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
      format.html { redirect_to prestamos_path, notice: "Prestamo was successfully destroyed.", status: :see_other }
      format.json { head :no_content }
    end
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

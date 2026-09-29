class AddFrecuenciaToPrestamos < ActiveRecord::Migration[8.1]
  def change
    add_column :prestamos, :frecuencia, :string, null: false, default: "semanal"
    remove_column :prestamos, :dias_entre_pagos, :integer, null: false, default: 7
  end
end

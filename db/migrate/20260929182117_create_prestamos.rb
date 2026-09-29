class CreatePrestamos < ActiveRecord::Migration[8.1]
  def change
    create_table :prestamos do |t|
      t.references :persona, null: false, foreign_key: true
      t.decimal :monto_prestado, precision: 12, scale: 2
      t.decimal :monto_a_devolver, precision: 12, scale: 2
      t.decimal :monto_pago, precision: 12, scale: 2
      t.date :fecha_primer_pago
      t.integer :dias_entre_pagos, null: false, default: 7

      t.timestamps
    end
  end
end

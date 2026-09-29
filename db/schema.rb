# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# This file is the source Rails uses to define your schema when running `bin/rails
# db:schema:load`. When creating a new database, `bin/rails db:schema:load` tends to
# be faster and is potentially less error prone than running all of your
# migrations from scratch. Old migrations may fail to apply correctly if those
# migrations use external dependencies or application code.
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema[8.1].define(version: 2026_09_29_184307) do
  create_table "personas", force: :cascade do |t|
    t.string "nombre"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
  end

  create_table "prestamos", force: :cascade do |t|
    t.integer "persona_id", null: false
    t.decimal "monto_prestado", precision: 12, scale: 2
    t.decimal "monto_a_devolver", precision: 12, scale: 2
    t.decimal "monto_pago", precision: 12, scale: 2
    t.date "fecha_primer_pago"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "frecuencia", default: "semanal", null: false
    t.index ["persona_id"], name: "index_prestamos_on_persona_id"
  end

  add_foreign_key "prestamos", "personas"
end

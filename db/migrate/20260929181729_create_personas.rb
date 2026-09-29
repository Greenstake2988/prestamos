class CreatePersonas < ActiveRecord::Migration[8.1]
  def change
    create_table :personas do |t|
      t.string :nombre

      t.timestamps
    end
  end
end

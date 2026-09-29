class Persona < ApplicationRecord
  validates :nombre, presence: true
end

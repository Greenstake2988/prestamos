class Persona < ApplicationRecord
  validates :nombre, presence: true

  has_many :prestamos, dependent: :restrict_with_error
end

class Context < ActiveRecord::Base
  validates_presence_of :name
  validates_length_of :name, :minimum => 3
  has_many :quotes, :order => 'created_at DESC'
end

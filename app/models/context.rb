class Context < ActiveRecord::Base
  validates_presence_of :name
  validates_length_of :name, :minimum => 3
  has_many :quotes, :order => 'created_at DESC'
  has_and_belongs_to_many :users, :uniq => true

  def add_user(user)
    users << user unless users.exists?(:id => user.id)
  end
end

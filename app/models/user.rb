class User < ActiveRecord::Base
  validates_presence_of :username
  validates_length_of :username, :minimum => 3
  validates_uniqueness_of :username
end

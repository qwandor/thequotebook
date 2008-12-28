class User < ActiveRecord::Base
  validates_presence_of :username
  validates_length_of :username, :minimum => 3
  validates_uniqueness_of :username, :case_sensitive => false
  has_many :said_quotes, :class_name => "Quote", :foreign_key => "quotee_id"
end

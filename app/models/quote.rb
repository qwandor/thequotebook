class Quote < ActiveRecord::Base
  belongs_to :context
  belongs_to :quoter, :class_name => 'User'
  belongs_to :quotee, :class_name => 'User'
end

class Quote < ActiveRecord::Base
  validates_presence_of :quoter, :quotee, :context
end

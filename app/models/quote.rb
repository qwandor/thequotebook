class Quote < ActiveRecord::Base
  validates_presence_of :quoter, :quotee, :context
  validates_length_of :quote_text, :minimum => 3
end

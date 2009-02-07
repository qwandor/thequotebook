class Comment < ActiveRecord::Base
  validates_presence_of :user, :quote, :body
end

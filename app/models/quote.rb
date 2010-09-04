class Quote < ActiveRecord::Base
  validates_presence_of :quoter, :quotee, :context
  validates_length_of :quote_text, :minimum => 3
  has_many :comments, :order => 'created_at ASC', :dependent => :delete_all

  after_create :send_notification

  #For pagination with will_paginate
  cattr_reader :per_page
  @@per_page = 10

protected
  #If quotee has email notification enabled, then send them an email telling them that they have been quoted
  def send_notification()
    if quotee.email_address && quotee.email_notification
      UserMailer.deliver_quote_email(self)
    end
  end
end

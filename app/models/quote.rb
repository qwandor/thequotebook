class Quote < ActiveRecord::Base
  validates_presence_of :quoter, :quotee, :context
  validates_length_of :quote_text, :minimum => 3

  after_create :send_notification

protected
  def send_notification()
    UserMailer.deliver_quote_email(self)
  end
end

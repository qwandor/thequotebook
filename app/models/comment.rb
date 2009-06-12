class Comment < ActiveRecord::Base
  validates_presence_of :user, :quote, :body
  validates_length_of :body, :minimum => 3

  after_create :send_notification

protected
  #If quotee or quoter have email notification enabled, then send them an email telling them about the comment
  def send_notification()
    [quote.quoter, quote.quotee].each do |recipient|
      if recipient.email_address && recipient.email_notification && recipient != user #Do not send emails to people about their own comments
        UserMailer.deliver_comment_email(self, recipient)
      end
    end
  end
end

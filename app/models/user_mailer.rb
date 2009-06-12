class UserMailer < ActionMailer::Base
  helper :application, :quotes, :users #Cannot do :all in ActionMailer

  #Send an email notification to the given user that they have been quoted
  def quote_email(quote)
    recipients quote.quotee.email_address
    from 'theQuotebook <notifications@thequotebook.net>'
    subject "#{quote.quoter.fullname} quoted you in #{quote.context.name}"
    sent_on Time.now
    body({:quote => quote})
  end

  #Send an email notification to the quoter and quotee that a new comment has been added to the quote
  def comment_email(comment, recipient)
    recipients recipient.email_address
    from 'theQuotebook <notifications@thequotebook.net>'
    subject "#{comment.user.fullname} commented on your quote in #{comment.quote.context.name}"
    sent_on Time.now
    body({:comment => comment, :recipient => recipient, :quote => comment.quote})
  end
end

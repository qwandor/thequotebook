class UserMailer < ActionMailer::Base
  helper :application, :quotes, :users #Cannot do :all in ActionMailer

  #If enabled, send an email notification to the given user that they have been quoted
  def quote_email(quote)
    user = quote.quotee
    if user.email_address && user.email_notification
      recipients user.email_address
      from 'theQuotebook <notifications@thequotebook.net>'
      subject "#{quote.quoter.fullname} quoted you in #{quote.context.name}"
      sent_on Time.now
      body({:quote => quote})
    end
  end
end

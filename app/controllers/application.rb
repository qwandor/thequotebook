# Filters added to this controller apply to all controllers in the application.
# Likewise, all the methods added will be available for all controllers.

class ApplicationController < ActionController::Base
  include AuthenticatedSystem

  helper :all # include all helpers, all the time

  # See ActionController::RequestForgeryProtection for details
  # Uncomment the :secret if you're not using the cookie session store
  protect_from_forgery :secret => '0af2f8aed03e9e73d9772a54d5cd75fa' #Change this on the server

  # See ActionController::Base for details
  # Uncomment this to filter the contents of submitted sensitive data parameters
  # from your application log (in this case, all fields with names like "password").
  # filter_parameter_logging :password

  before_filter :set_time_zone, :get_random_quote

protected
  def set_time_zone
    Time.zone = current_user ? current_user.time_zone : 'Wellington'
  end

  def get_random_quote
    if logged_in?
      @random_quote = Quote.first(:conditions => ['context_id IN (?) AND NOT hidden', current_user.context_ids], :order => 'random()')
    end
    if @random_quote.nil? #Either not logged in, or user is not a member of any quotebooks, or their quotebooks have no quotes
      @random_quote = Quote.first(:conditians => ['NOT hidden'], :order => 'random()')
    end
  end
end

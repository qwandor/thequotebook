# This controller handles the login/logout function of the site.
class SessionsController < ApplicationController
  protect_from_forgery :except => [:create] #Seems to be necessary, as OpenID provider does not pass authenticity_token back
  # TODO: Would it be better to modify the client library to pass this on in the openid.return_to url sent to the provider?

  # render new.rhtml
  def new
    @remember_me = true
  end

  def create
    logout_keeping_session!

    authenticate_with_open_id(params[:openid] || params[:openid_identifier], :required => [:nickname, :fullname, 'http://axschema.org/namePerson/friendly', 'http://axschema.org/namePerson', 'http://axschema.org/contact/email'], :optional => [:email], :return_to => url_for(:controller => 'sessions', :action => 'create', :_method => 'post', :only_path => false, :remember_me => params[:remember_me])) do |result, openid, registration|
      if result.successful?
        if user = User.find_by_openid(openid)
          successful_login(user)
        else
          #No such user yet, let them create an account
          session[:new_user_openid] = openid
          session[:new_user_registration] = registration
          redirect_to new_user_path
        end
      else
        failed_login result.message
      end
    end
  end

  def destroy
    logout_killing_session!
    flash[:notice] = "You have been logged out."
    redirect_back_or_default('/')
  end

protected
  # Track failed login attempts
  def note_failed_signin(message)
    flash[:error] = "Couldn't log you in: #{message}"
    logger.warn "Failed login for '#{params[:openid]}' from #{request.remote_ip} at #{Time.now.utc}: #{message}"
  end

  def failed_login(message)
    note_failed_signin(message)
    @openid      = params[:openid]
    @remember_me = params[:remember_me]
    render :action => 'new'
  end

  def successful_login(user)
    # Protects against session fixation attacks, causes request forgery
    # protection if user resubmits an earlier form using back
    # button. Uncomment if you understand the tradeoffs.
    # reset_session
    self.current_user = user
    new_cookie_flag = (params[:remember_me] == "1")
    handle_remember_cookie! new_cookie_flag
    redirect_back_or_default('/')
    flash[:notice] = "Logged in successfully"
  end
end

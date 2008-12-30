module AuthenticatedTestHelper
  # Sets the current usex in the session from the usex fixtures.
  def login_as(usex)
    @request.session[:usex_id] = usex ? usexes(usex).id : nil
  end

  def authorize_as(usex)
    @request.env["HTTP_AUTHORIZATION"] = usex ? ActionController::HttpAuthentication::Basic.encode_credentials(usexes(usex).login, 'monkey') : nil
  end
  
end

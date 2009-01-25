class UsersController < ApplicationController
  before_filter :find_user, :only => [:show, :edit, :update, :destroy, :quotes]
  before_filter :login_required, :only => [:edit, :update]
  before_filter :own_account, :only => [:edit, :update]

  # GET /users
  # GET /users.xml
  def index
    @users = User.find(:all)

    respond_to do |format|
      format.html # index.html.erb
      format.xml  { render :xml => @users }
    end
  end

  # GET /users/1
  # GET /users/1.xml
  def show
    respond_to do |format|
      format.html # show.html.erb
      format.xml  { render :xml => @user }
    end
  end

  # GET /users/1/quotes
  # GET /users/1/quotes.xml
  # GET /users/1/quotes.atom
  def quotes
    order = params[:format] == 'atom' ? 'updated_at DESC' : 'created_at DESC'
    @quotes = Quote.find(:all, :conditions => ['quotee_id = ?', @user.id], :order => order)

    @feed_title = "Quoteyou: Quotes by #{@user.fullname}"

    respond_to do |format|
      format.html
      format.xml  { render :xml => @quotes }
      format.atom { render :template => 'quotes/index' }
    end
  end

  # GET /users/new
  # GET /users/new.xml
  def new
    if params[:mode] == 'partial'
      #User is being created by an existing user, so that they can be quoted
      login_required || return

      @user = User.new
      @user.fullname = params[:fullname]

      respond_to do |format|
        format.html { render :action => 'new_partial' }
        format.xml  { render :xml => @user }
      end
    else
      #A new user is creating an account for themselves normally
      if session[:new_user_openid].nil?
        flash[:notice] = 'To register a new account, please login with your OpenID.'
        redirect_to new_session_path and return
      end

      @user = User.new
      @user.openid = session[:new_user_openid]
      regdata = session[:new_user_registration]
      @user.username = regdata['nickname'] || regdata['http://axschema.org/namePerson/friendly']
      @user.fullname = regdata['fullname'] || regdata['http://axschema.org/namePerson']
      @user.email_address = regdata['email'] || regdata['http://axschema.org/contact/email']

      #Clear session out now that we are done with it
      session[:new_user_openid] = nil
      session[:new_user_registration] = nil

      respond_to do |format|
        format.html # new.html.erb
        format.xml  { render :xml => @user }
      end
    end
  end

  # GET /users/1/edit
  def edit
  end

  # POST /users
  # POST /users.xml
  def create
    params[:user][:email_address] = nil if params[:user][:email_address].empty?

    if params[:mode] == 'partial'
      #User is being created by an existing user, so that they can be quoted
      login_required || return

      @user = User.new(params[:user])
      @user.openid = nil
      @user.username = nil
      respond_to do |format|
        if @user.save
          #Return to add quote page, or whereever we were
          flash[:notice] = 'User added.'
          format.html { redirect_back_or_default(users_path) }
          format.xml  { render :xml => @user, :status => :created, :location => @user }
        else
          format.html { render :action => 'new_partial' }
          format.xml  { render :xml => @user.errors, :status => :unprocessable_entity }
        end
      end
    else
      #A new user is creating an account for themselves normally
      if session[:new_user_openid].nil?
        flash[:notice] = 'To register a new account, please login with your OpenID.'
        redirect_to new_session_path and return
      end

      #Ensure that username is set, in case someone tries to send their own post request without them whereby to create a user without a username (nil usernames are allow by model so that they can be used for partial users, but we do not want to allow them for real users)
      params[:user][:username] ||= ''

      logout_keeping_session!

      if params[:mode] == 'claimpartial'
        #The account already exists as a partial user, so they are claiming it
        @user = User.find(params[:id])
        @user.attributes = params[:user]
        @user.openid = session[:new_user_openid]
      else
        #Check for similar existing users, which might have been created by another user quoting them
        @possible_matches = User.all(:conditions => ['openid IS NULL AND (LOWER(email_address) = LOWER(?) OR LOWER(fullname) = LOWER(?))', params[:user][:email_address], params[:user][:fullname]])

        @user = User.new(params[:user])
        @user.openid = session[:new_user_openid]

        if !@possible_matches.empty?
          render :action => 'create_matches'
          return
        end
      end

      respond_to do |format|
        if @user && @user.save && @user.errors.empty?
          # Protects against session fixation attacks, causes request forgery
          # protection if visitor resubmits an earlier form using back
          # button. Uncomment if you understand the tradeoffs.
          # reset session
          self.current_user = @user # !! now logged in
          flash[:notice] = "User was successfully #{params[:mode] == 'claimpartial' ? 'claimed' : 'created'}. Thanks for signing up!"
          format.html { redirect_back_or_default('/') }
          format.xml  { render :xml => @user, :status => :created, :location => @user }
        else
          flash[:error]  = 'We couldn\'t set up that account, sorry. Please try again, or contact an admin.'
          format.html { render :action => "new" }
          format.xml  { render :xml => @user.errors, :status => :unprocessable_entity }
        end
      end
    end
  end

  # PUT /users/1
  # PUT /users/1.xml
  def update
    params[:user][:username] ||= ''

    respond_to do |format|
      if @user.update_attributes(params[:user])
        flash[:notice] = 'User was successfully updated.'
        format.html { redirect_to(@user) }
        format.xml  { head :ok }
      else
        format.html { render :action => "edit" }
        format.xml  { render :xml => @user.errors, :status => :unprocessable_entity }
      end
    end
  end

  # DELETE /users/1
  # DELETE /users/1.xml
  def destroy
    # TODO: Allow admins to use this, perhaps (disabled completely for now)
    return

    @user.destroy

    respond_to do |format|
      format.html { redirect_to(users_url) }
      format.xml  { head :ok }
    end
  end

protected
  def permission_denied(reason = 'Permission denied')
    respond_to do |format|
      format.html do
        flash[:error] = reason
        redirect_to(users_url)
      end
      format.xml do
        render :xml => reason, :status => :forbidden
      end
    end
  end

  def find_user
    @user ||= User.find(params[:id])
  end

  def own_account
    find_user && ((logged_in? && @user == current_user) || permission_denied('You may only edit your own user account'))
  end
end

class QuotesController < ApplicationController
  before_filter :find_quote, :only => [:show, :edit, :update, :destroy]
  before_filter :login_required, :only => [:new, :create]
  before_filter :own_quote, :only => [:edit, :update, :destroy]
  after_filter :store_location, :only => [:new, :update]

  auto_complete_for :context, :name
  protect_from_forgery :except => [:auto_complete_for_context_name, :auto_complete_for_quotee_name]

  def auto_complete_for_quotee_name
    @term = params[:quotee][:name].downcase
    @items = User.all({:conditions => ['fullname ILIKE ? OR username ILIKE ?', "%#{@term}%", "%#{@term}%"], :order => 'username ASC, fullname ASC', :limit => 10})
    # TODO: Do we want to show username rather than fullname if that was what we matched, or even for all users with one?
    render :inline => "<%= auto_complete_result(@items, 'fullname', @term) %>"
  end

  layout 'no_sidebars'

  # GET /quotes
  # GET /quotes.xml
  # GET /quotes.atom
  def index
    order = params[:format] == 'atom' ? 'updated_at DESC' : 'created_at DESC'
    @quotes = Quote.all(:conditions => ['NOT hidden'], :order => order)

    @feed_title = 'theQuotebook: All quotes'

    respond_to do |format|
      format.html # index.html.erb
      format.xml  { render :xml => @quotes }
      format.atom { render :layout => false } # index.atom.builder
    end
  end

  # GET /quotes/1
  # GET /quotes/1.xml
  def show
    respond_to do |format|
      format.html # show.html.erb
      format.xml  { render :xml => @quote.to_xml(:include => [:context, :quotee, :quoter]) }
    end
  end

  # GET /quotes/new
  # GET /quotes/new.xml
  def new
    @quote = session[:quote_in_progress] || Quote.new
    session[:quote_in_progress] = nil
    @quote.context = Context.find(params[:context]) if params[:context]

    respond_to do |format|
      format.html # new.html.erb
      format.xml  { render :xml => @quote }
    end
  end

  # GET /quotes/1/edit
  def edit
  end

  # POST /quotes
  # POST /quotes.xml
  def create
    properties = params[:quote]

    properties[:quotee], @possible_quotee_matches = User.find_from_string(params[:quotee][:name], current_user)

    properties[:context] = Context.first(:conditions => ['name = ?', params[:context][:name]])
    # TODO: give helpful error if quotee or context is nil (i.e. could not match name), rather than just error about it being blank

    @quote = Quote.new(properties)
    @quote.quoter = current_user

    #Store the quote as it is being edited, so that we can return to editing it if we have to stop to add a new user in the middle
    session[:quote_in_progress] = @quote if @possible_quotee_matches

    respond_to do |format|
      if !@possible_quotee_matches && @quote.save
        flash[:notice] = 'Quote was successfully created.'

        #While we are at it, add quoter and quotee to context
        properties[:context].add_user(current_user)
        properties[:context].add_user(properties[:quotee])

        format.html { redirect_to(@quote) }
        format.xml  { render :xml => @quote, :status => :created, :location => @quote }
      else
        format.html { render :action => 'new' }
        format.xml  { render :xml => @quote.errors, :status => :unprocessable_entity }
      end
    end
  end

  # PUT /quotes/1
  # PUT /quotes/1.xml
  def update
    properties = params[:quote]
    properties.delete(:quoter) #Cannot change quoter

    properties[:quotee], @possible_quotee_matches = User.find_from_string(params[:quotee][:name], current_user)

    properties[:context] = Context.first(:conditions => ['name = ?', params[:context][:name]])

    respond_to do |format|
      if !@possible_quotee_matches && @quote.update_attributes(properties)
        flash[:notice] = 'Quote was successfully updated.'
        format.html { redirect_to(@quote) }
        format.xml  { head :ok }
      else
        format.html { render :action => "edit" }
        format.xml  { render :xml => @quote.errors, :status => :unprocessable_entity }
      end
    end
  end

  # DELETE /quotes/1
  # DELETE /quotes/1.xml
  def destroy
    @quote.destroy

    respond_to do |format|
      format.html { redirect_to(@quote.context) }
      format.xml  { head :ok }
    end
  end

protected
  def find_quote
    @quote ||= Quote.find(params[:id])
  end

  def own_quote
    find_quote && ((logged_in? && @quote.quoter == current_user) || access_denied)
  end
end

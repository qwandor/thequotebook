class QuotesController < ApplicationController
  before_filter :find_quote, :only => [:show, :edit, :update, :destroy]
  before_filter :login_required, :only => [:new, :create]
  before_filter :own_quote, :only => [:edit, :update, :destroy]

  # GET /quotes
  # GET /quotes.xml
  def index
    @quotes = Quote.find(:all)

    respond_to do |format|
      format.html # index.html.erb
      format.xml  { render :xml => @quotes }
    end
  end

  # GET /quotes/1
  # GET /quotes/1.xml
  def show
    respond_to do |format|
      format.html # show.html.erb
      format.xml  { render :xml => @quote }
    end
  end

  # GET /quotes/new
  # GET /quotes/new.xml
  def new
    @quote = Quote.new
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
    properties[:quoter] = current_user

    # TODO: Move this to a method on User
    quotee_string = properties[:quotee]
    properties[:quotee] = User.first(:conditions => ['username = ?', quotee_string])
    properties[:quotee] = User.first(:conditions => ['fullname = ?', quotee_string]) if properties[:quotee].nil?

    properties[:context] = Context.first(:conditions => ['name = ?', properties[:context]])

    @quote = Quote.new(properties)

    respond_to do |format|
      if @quote.save
        flash[:notice] = 'Quote was successfully created.'
        format.html { redirect_to(@quote) }
        format.xml  { render :xml => @quote, :status => :created, :location => @quote }
      else
        format.html { render :action => "new" }
        format.xml  { render :xml => @quote.errors, :status => :unprocessable_entity }
      end
    end
  end

  # PUT /quotes/1
  # PUT /quotes/1.xml
  def update
    properties = params[:quote]
    properties.delete(:quoter)

    # TODO: Move this to a method on User
    quotee_string = properties[:quotee]
    properties[:quotee] = User.first(:conditions => ['username = ?', quotee_string])
    properties[:quotee] = User.first(:conditions => ['fullname = ?', quotee_string]) if properties[:quotee].nil?

    properties[:context] = Context.first(:conditions => ['name = ?', properties[:context]])

    respond_to do |format|
      if @quote.update_attributes(properties)
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
      format.html { redirect_to(quotes_url) }
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

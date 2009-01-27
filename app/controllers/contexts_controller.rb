class ContextsController < ApplicationController
  before_filter :find_context, :only => [:show, :edit, :update, :destroy, :join, :leave, :quotes]
  before_filter :login_required, :only => [:new, :create, :edit, :update, :destroy, :join, :leave]

  # GET /contexts
  # GET /contexts.xml
  def index
    @contexts = Context.find(:all)

    respond_to do |format|
      format.html # index.html.erb
      format.xml  { render :xml => @contexts }
    end
  end

  # GET /contexts/1
  # GET /contexts/1.xml
  def show
    respond_to do |format|
      format.html # show.html.erb
      format.xml  { render :xml => @context }
    end
  end

  # GET /contexts/new
  # GET /contexts/new.xml
  def new
    @context = Context.new

    respond_to do |format|
      format.html # new.html.erb
      format.xml  { render :xml => @context }
    end
  end

  # GET /contexts/1/edit
  def edit
  end

  # POST /contexts
  # POST /contexts.xml
  def create
    @context = Context.new(params[:context])
    @context.add_user(current_user) #Join it while we are at it

    respond_to do |format|
      if @context.save
        flash[:notice] = 'Context was successfully created.'
        format.html { redirect_to(@context) }
        format.xml  { render :xml => @context, :status => :created, :location => @context }
      else
        format.html { render :action => "new" }
        format.xml  { render :xml => @context.errors, :status => :unprocessable_entity }
      end
    end
  end

  # PUT /contexts/1
  # PUT /contexts/1.xml
  def update
    respond_to do |format|
      if @context.update_attributes(params[:context])
        flash[:notice] = 'Context was successfully updated.'
        format.html { redirect_to(@context) }
        format.xml  { head :ok }
      else
        format.html { render :action => "edit" }
        format.xml  { render :xml => @context.errors, :status => :unprocessable_entity }
      end
    end
  end

  # DELETE /contexts/1
  # DELETE /contexts/1.xml
  def destroy
    @context.destroy

    respond_to do |format|
      format.html { redirect_to(contexts_url) }
      format.xml  { head :ok }
    end
  end

  # GET /contexts/1/latest
  # GET /contexts/1/latest.xml
  def latest #Show latest quote in this context
    @quote = Quote.first(:conditions => ['context_id = ?', params[:id]], :order => 'created_at DESC')
    respond_to do |format|
      format.html { render :template => 'quotes/show' }
      format.xml  { render :xml => @quote.to_xml(:include => [:context, :quotee, :quoter]) }
    end
  end

  # POST /contexts/1/join
  def join #Add the current user to the context
    respond_to do |format|
      if @context.add_user(current_user)
        flash[:notice] = "You are now a member of #{@context.name}."
        format.html { redirect_to(@context) }
        format.xml  { head :ok }
      else
        flash[:error] = 'Error joining context'
        format.html { redirect_to(@context) }
        format.xml  { render :xml => @context.errors, :status => :unprocessable_entity }
      end
    end
  end

  # POST /contexts/1/leave
  def leave #Remove the current user from the context
    respond_to do |format|
      if @context.users.delete(current_user)
        flash[:notice] = "You are no longer a member of #{@context.name}."
        format.html { redirect_to(@context) }
        format.xml  { head :ok }
      else
        flash[:error] = 'Error leaving context'
        format.html { redirect_to(@context) }
        format.xml  { render :xml => @context.errors, :status => :unprocessable_entity }
      end
    end
  end

  # GET /contexts/1/quotes
  # GET /contexts/1/quotes.xml
  # GET /contexts/1/quotes.atom
  def quotes
    order = params[:format] == 'atom' ? 'updated_at DESC' : 'created_at DESC'
    @quotes = Quote.find(:all, :conditions => ['context_id = ?', @context.id], :order => order)

    @feed_title = "Quoteyou: #{@context.name} quotes"

    respond_to do |format|
      format.html
      format.xml  { render :xml => @quotes.to_xml(:include => [:quotee, :quoter]) }
      format.atom { render :template => 'quotes/index' }
    end
  end

protected
  def find_context
    @context ||= Context.find(params[:id])
  end
end

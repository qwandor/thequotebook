class ContextsController < ApplicationController
  before_filter :find_context, :only => [:show, :edit, :update, :destroy]
  before_filter :login_required, :only => [:new, :create, :edit, :update, :destroy]

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
  def latest #Show latest quote in this context
    @quote = Quote.first(:conditions => ['context_id = ?', params[:id]], :order => 'created_at DESC')
    respond_to do |format|
      format.html { render :template => 'quotes/show' }
      format.xml  { render :xml => @quote }
    end
  end

protected
  def find_context
    @context ||= Context.find(params[:id])
  end
end

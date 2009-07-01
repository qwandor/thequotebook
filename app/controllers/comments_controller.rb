class CommentsController < ApplicationController
  before_filter :find_quote
  before_filter :find_comment, :only => [:show, :edit, :update, :destroy]
  before_filter :login_required, :only => [:new, :create]
  before_filter :own_comment, :only => [:edit, :update, :destroy]

  # GET /quotes/1/comments
  # GET /quotes/1/comments.xml
  # GET /quotes/1/comments.atom
  def index
    @feed_title = "theQuotebook: Comments on #{@quote.quote_text}"

    respond_to do |format|
      format.html # index.html.erb
      format.xml  { render :xml => @quote.comments.to_xml(:include => [:user]) }
      format.atom { @comments = @quote.comments } # index.atom.builder
    end
  end

  # GET /quotes/1/comments/1
  # GET /quotes/1/comments/1.xml
  def show
    respond_to do |format|
      format.html # show.html.erb
      format.xml  { render :xml => @comment.to_xml(:include => [:user]) }
    end
  end

  # GET /quotes/1/comments/new
  # GET /quotes/1/comments/new.xml
  def new
    @comment = @quote.comments.build

    respond_to do |format|
      format.html # new.html.erb
      format.xml  { render :xml => @comment }
    end
  end

  # GET /quotes/1/comments/1/edit
  def edit
  end

  # POST /quotes/1/comments
  # POST /quotes/1/comments.xml
  def create
    @comment = @quote.comments.build(params[:comment])
    @comment.user = current_user

    respond_to do |format|
      if @comment.save
        flash[:notice] = 'Comment posted.'
        format.html { redirect_to @quote }
        format.xml  { render :xml => @comment, :status => :created, :location => @comment }
      else
        format.html { render :action => 'new' }
        format.xml  { render :xml => @comment.errors, :status => :unprocessable_entity }
      end
    end
  end

  # PUT /quotes/1/comments/1
  # PUT /quotes/1/comments/1.xml
  def update
    params[:comment].delete(:user) #Cannot change commenter

    respond_to do |format|
      if @comment.update_attributes(params[:comment])
        flash[:notice] = 'Comment was successfully updated.'
        format.html { redirect_to @quote }
        format.xml  { head :ok }
      else
        format.html { render :action => 'edit' }
        format.xml  { render :xml => @comment.errors, :status => :unprocessable_entity }
      end
    end
  end

  # DELETE /quotes/1/comments/1
  # DELETE /quotes/1/comments/1.xml
  def destroy
    @comment.destroy

    respond_to do |format|
      format.html { redirect_to quote_comments_path(@quote) }
      format.xml  { head :ok }
    end
  end

protected
  def find_quote
    @quote ||= Quote.find(params[:quote_id])
  end

  def find_comment
    @comment = @quote.comments.find(params[:id])
  end

  def own_comment
    find_comment && ((logged_in? && @comment.user == current_user) || access_denied)
  end
end

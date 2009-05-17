class HomeController < ApplicationController
  def index
    @latest_quotes = Quote.all(:order => 'created_at desc', :limit => 5)
    @top_contexts = Context.all(:select => 'id, name, description, (select count(*) from quotes where context_id = contexts.id) as quote_count', :order => 'quote_count desc', :limit => 5)
    if logged_in?
      @current_user_contexts = current_user.contexts
      @current_user_quotes = Quote.all(:conditions => ['context_id in (?)', current_user.context_ids], :order => 'created_at desc', :limit => 5)
      @current_user_comments = Comment.all(:joins => 'INNER JOIN quotes ON quotes.id=comments.quote_id', :conditions => ['context_id in (?)', current_user.context_ids], :order => 'created_at desc', :limit => 5)
    end
  end
end

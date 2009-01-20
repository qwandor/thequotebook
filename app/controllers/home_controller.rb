class HomeController < ApplicationController
  def index
    @latest_quotes = Quote.all(:order => 'created_at desc', :limit => 5)
    @top_contexts = Context.all(:select => 'id, name, description, (select count(*) from quotes where context_id = contexts.id) as quote_count', :order => 'quote_count desc', :limit => 3)
    if logged_in?
      @current_user_contexts = Context.all(:select => 'id, name, description, (select count(*) from quotes where context_id = contexts.id) as quote_count', :conditions => ['(select count(*) from quotes where contexts.id = context_id and ? in (quotee_id, quoter_id)) > 0', current_user.id], :order => 'quote_count desc')
      @current_user_quotes = Quote.all(:conditions => ['context_id in (?)', @current_user_contexts.map{|context| context.id}], :order => 'created_at desc', :limit => 5)
    end
  end
end

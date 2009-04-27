xml.instruct!

xml.feed 'xmlns' => 'http://www.w3.org/2005/Atom' do
  xml.title   @feed_title
  xml.link    'rel' => 'self', 'type' => 'application/atom+xml', "href" => url_for(:only_path => false, :format => :atom)
  xml.link    'rel' => 'alternate', 'type' => 'text/html', 'href' => url_for(:only_path => false)
  xml.id      url_for(:only_path => false, :format => 'atom')
  xml.updated @quotes.first.updated_at.strftime '%Y-%m-%dT%H:%M:%SZ' if @quotes.any?
  xml.generator 'theQuotebook', :uri => url_for(:only_path => false, :controller => 'home')

  @quotes.each do |quote|
    xml.entry do
      xml.title   "#{quote.quotee.fullname}: #{quote.quote_text}"
      xml.link    'rel' => 'alternate', 'type' => 'text/html', 'href' => url_for(:only_path => false, :controller => 'quotes', :action => 'show', :id => quote.id)
      xml.id      url_for(:only_path => false, :controller => 'quotes', :action => 'show', :id => quote.id) # TODO: better ids (maybe tag URLs)
      xml.updated quote.updated_at.strftime '%Y-%m-%dT%H:%M:%SZ'
      xml.published quote.created_at.strftime '%Y-%m-%dT%H:%M:%SZ'
      xml.author do
         xml.name quote.quoter.username
         xml.uri url_for(:only_path => false, :controller => 'users', :action => 'show', :id => quote.quoter.id)
      end
      xml.content 'type' => 'html' do
        xml.text! chatty_quote(quote) + "\n"
      end
    end
  end
end

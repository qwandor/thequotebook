xml.instruct!

xml.feed 'xmlns' => 'http://www.w3.org/2005/Atom' do
  xml.title   @feed_title
  xml.link    'rel' => 'self', 'type' => 'application/atom+xml', "href" => url_for(:only_path => false, :format => :atom)
  xml.link    'rel' => 'alternate', 'type' => 'text/html', 'href' => url_for(:only_path => false)
  xml.id      url_for(:only_path => false, :format => 'atom')
  xml.updated @comments.first.updated_at.strftime '%Y-%m-%dT%H:%M:%SZ' if @comments.any?
  xml.generator 'theQuotebook', :uri => url_for(:only_path => false, :controller => 'home')

  @comments.each do |comment|
    xml.entry do
      url = url_for(:only_path => false, :controller => 'comments', :action => 'show', :quote_id => comment.quote.id, :id => comment.id)
      xml.title   "#{comment.user.username} on #{comment.quote.quote_text} (#{comment.quote.quotee.fullname})"
      xml.link    'rel' => 'alternate', 'type' => 'text/html', 'href' => url
      xml.id      url # TODO: better ids (maybe tag URLs)
      xml.updated comment.updated_at.strftime '%Y-%m-%dT%H:%M:%SZ'
      xml.published comment.created_at.strftime '%Y-%m-%dT%H:%M:%SZ'
      xml.author do
         xml.name comment.user.username
         xml.uri url_for(:only_path => false, :controller => 'users', :action => 'show', :id => comment.user.id)
      end
      xml.content 'type' => 'html' do
        xml.text! "<p>
  On #{link_to comment.created_at, url},
  #{link_to_user comment.user} commented on #{link_to_user comment.quote.quotee}'s quote #{link_to trim_if_needed(quote_marks_if_needed(comment.quote.quote_text), 40).gsub(/[\r\n]+/, ' ').bbcode_to_html({}, :enable, true, false, [:bold, :italics]), comment.quote}:
</p>
#{comment.body.bbcode_to_html}
\n"
      end
    end
  end
end

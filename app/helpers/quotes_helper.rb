module QuotesHelper
  #Nicely formatted version of the quote ready to display
  #May include various links
  def formatted_quote(quote, options={})
    raise "Invalid quote" unless quote
    options.reverse_merge! :quote_link => true, :quoter_link => true, :quotee_link => true, :show_context => true
    quote_link = options.delete(:quote_link)
    quoter_link = options.delete(:quoter_link)
    quotee_link = options.delete(:quotee_link)
    show_context = options.delete(:show_context)

    "<p class=\"quote\">On #{h quote.created_at}, #{link_to_user quote.quoter, :actually_link => quoter_link} quoted #{link_to_user quote.quotee, :actually_link => quotee_link} as saying \"#{quote_link ? link_to(h(quote.quote_text), quote) : h(quote.quote_text)}\"" + (show_context ? " in #{link_to h(quote.context.name), quote.context}." : '') + "<\p>"
  end

  def chatty_quote(quote, options={})
    raise "Invalid quote" unless quote
    options.reverse_merge! :quote_link => true, :quoter_link => true, :quotee_link => true, :show_context => true
    quote_link = options.delete(:quote_link)
    quoter_link = options.delete(:quoter_link)
    quotee_link = options.delete(:quotee_link)
    show_context = options.delete(:show_context)

    "On #{h quote.created_at}, #{link_to_user quote.quoter, :actually_link => quoter_link} quoted #{link_to_user quote.quotee, :actually_link => quotee_link} as saying \"#{quote_link ? link_to(h(quote.quote_text), quote) : h(quote.quote_text)}\"" + (show_context ? " in #{link_to h(quote.context.name), quote.context}." : '')
  end
end

require 'bb-ruby'

module QuotesHelper
  #Nicely formatted version of the quote ready to display
  #May include various links
  def formatted_quote(quote, options={})
    raise "Invalid quote" unless quote
    options.reverse_merge! :quote_link => true, :quoter_link => true, :quotee_link => true, :show_context => true, :show_comments => true
    quote_link = options.delete(:quote_link)
    quoter_link = options.delete(:quoter_link)
    quotee_link = options.delete(:quotee_link)
    show_context = options.delete(:show_context)
    show_comments = options.delete(:show_comments)

    render :partial => 'shared/quote', :locals => {:quote => quote, :quote_link => quote_link, :quoter_link => quoter_link, :quotee_link => quotee_link, :show_context => show_context, :show_comments => show_comments}
  end

  def chatty_quote(quote, options={})
    raise "Invalid quote" unless quote
    options.reverse_merge! :quote_link => true, :quoter_link => true, :quotee_link => true, :show_context => true
    quote_link = options.delete(:quote_link)
    quoter_link = options.delete(:quoter_link)
    quotee_link = options.delete(:quotee_link)
    show_context = options.delete(:show_context)

    quote_text = quote_marks_if_needed(quote.quote_text).bbcode_to_html({}, :enable, true, false, [:bold, :italics])

    "On #{h quote.created_at}, #{link_to_user quote.quoter, :actually_link => quoter_link} quoted #{link_to_user quote.quotee, :actually_link => quotee_link} as saying #{quote_link ? link_to(quote_text, quote) : quote_text}" + (show_context ? " in #{link_to h(quote.context.name), quote.context}." : '')
  end

  def quote_marks_if_needed(text)
    if text.include?('"')
      text
    else
      '"' + text + '"'
    end
  end
end

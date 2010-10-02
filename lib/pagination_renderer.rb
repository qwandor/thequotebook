class PaginationListLinkRenderer < WillPaginate::LinkRenderer
  def initialize()
    @gap_marker = '<li><span class="gap">&hellip;</span></li>'
  end

  def to_html()
    html = windowed_links().join(@options[:separator])
    @options[:container] ? @template.content_tag(:ul, html, html_attributes) : html
  end

protected
  def windowed_links()
    links = []
    prev = nil
    visible_page_numbers.each do |n|
      links << gap_marker if prev and n > prev + 1
      links << page_link_or_span(n, 'button')
      prev = n
    end
    links
  end

  def page_link_or_span(page, span_class, text = nil)
    text ||= page.to_s
    if page && page != current_page
      page_link(page, text, :class => span_class)
    else
      page_span(page, text, :class => span_class)
    end
  end

  def page_link(page, text, attributes = {})
    @template.content_tag(:li, @template.link_to(text, url_for(page), attributes))
  end

  def page_span(page, text, attributes = {})
    @template.content_tag(:li, @template.content_tag(:span, text, attributes))
  end
end

module WillPaginate::ViewHelpers
  #Like will_paginate, but works even if there is only one page, and uses the custom renderer above
  def my_paginate(collection)
    renderer = PaginationListLinkRenderer.new()
    options = {:inner_window => 1, :outer_window => 0, :container => false}
    options = options.symbolize_keys.reverse_merge WillPaginate::ViewHelpers.pagination_options
    renderer.prepare(collection, options, self)
    renderer.to_html
  end
end

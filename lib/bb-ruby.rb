#Based on bb-ruby, http://github.com/ferblape/bb-ruby/
# Original code:
# Copyright (c) 2008 Craig P Jolicoeur, Fernando Blat
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#This modified file is licensed under same license as the rest of quoteyou, namely the GPL

module BBRuby
  @@imageformats = 'png|bmp|jpg|gif|jpeg'

  @@tags = {
    # tag name => [regex, replace, description, example, enable/disable symbol]
    'Bold' => [
      /\[strong(:.*)?\](.*?)\[\/strong\1?\]/mi,
      '<strong>\2</strong>',
      'Embolden text',
      'Look [strong]here[/strong]',
      :bold],
    'Italics' => [
      /\[em(:.+)?\](.*?)\[\/em\1?\]/mi,
      '<em>\2</em>',
      'Italicize or emphasize text',
      'Even my [em]cat[/em] was chasing the mailman!',
      :italics],
    'Bold (alternative)' => [
      /\[b(:.*)?\](.*?)\[\/b\1?\]/mi,
      '<strong>\2</strong>',
      'Embolden text',
      'Look [b]here[/b]',
      :bold],
    'Italics (alternative)' => [
      /\[i(:.+)?\](.*?)\[\/i\1?\]/mi,
      '<em>\2</em>',
      'Italicize or emphasize text',
      'Even my [i]cat[/i] was chasing the mailman!',
      :italics],
    'Underline' => [
      /\[u(:.+)?\](.*?)\[\/u\1?\]/mi,
      '<u>\2</u>',
      'Underline',
      'Use it for [u]important[/u] things or something',
      :underline],
    'Bold (easy)' => [
      /\*(.+)\*/mi,
      '<strong>\1</strong>',
      'Embolden text',
      'Look *here*',
      :bold],
    'Italics (easy)' => [
      /_(.+)_/mi,
      '<em>\1</em>',
      'Italicize or emphasize text',
      'Even my _cat_ was chasing the mailman!',
      :italics],
    'Strikeout' => [
      /\[s(:.+)?\](.*?)\[\/s\1?\]/mi,
      '<del>\2</del>',
      'Strikeout',
      '[s]nevermind[/s]',
      :delete],
    'Delete' => [
      /\[del(:.+)?\](.*?)\[\/del\1?\]/mi,
      '<del>\2</del>',
      'Deleted text',
      '[del]deleted text[/del]',
      :delete],
    'Insert' => [
      /\[ins(:.+)?\](.*?)\[\/ins\1?\]/mi,
      '<ins>\2</ins>',
      'Inserted Text',
      '[ins]inserted text[/del]',
      :insert],
    'Code' => [
      /\[code(:.+)?\](.*?)\[\/code\1?\]/mi,
      '<code>\2</code>',
      'Code Text',
      '[code]some code[/code]',
      :code],
    'Size' => [
      /\[size=['"]?(.*?)['"]?\](.*?)\[\/size\]/im,
      '<span style="font-size: \1px;">\2</span>',
      'Change text size',
      '[size=20]Here is some larger text[/size]',
      :size],
    'Color' => [
      /\[color=['"]?(\w+|\#\w{6})['"]?(:.+)?\](.*?)\[\/color\2?\]/im,
      '<span style="color: \1;">\3</span>',
      'Change text color',
      '[color=red]This is red text[/color]',
      :color],
    'Ordered List' => [
      /\[ol\](.*?)\[\/ol\]/mi,
      '<ol>\1</ol>',
      'Ordered list',
      'My favorite people (alphabetical order): [ol][li]Jenny[/li][li]Alex[/li][li]Beth[/li][/ol]',
      :orderedlist],
    'Unordered List' => [
      /\[ul\](.*?)\[\/ul\]/mi,
      '<ul>\1</ul>',
      'Unordered list',
      'My favorite people (order of importance): [ul][li]Jenny[/li][li]Alex[/li][li]Beth[/li][/ul]',
      :unorderedlist],
    'List Item' => [
      /\[li\](.*?)\[\/li\]/mi,
      '<li>\1</li>',
      'List item',
      'See ol or ul',
      :listitem],
    'List Item (alternative)' => [
      /\[\*(:[^\[]+)?\]([^(\[|\<)]+)/mi,
      '<li>\2</li>',
      'List item (alternative)',
      nil, nil,
      :listitem],
    'Unordered list (alternative)' => [
      /\[list(:.*)?\]((?:(?!list).)*)\[\/list(:.)?\1?\]/mi,
      '<ul>\2</ul>',
      'Unordered list item',
      '[list][*]item 1[*] item2[/list]',
      :list],
    'Ordered list (numerical)' => [
      /\[list=1(:.*)?\](.+)\[\/list(:.)?\1?\]/mi,
      '<ol>\2</ol>',
      'Ordered list numerically',
      '[list=1][*]item 1[*] item2[/list]',
      :list],
    'Ordered list (alphabetical)' => [
      /\[list=a(:.*)?\](.+)\[\/list(:.)?\1?\]/mi,
      '<ol sytle="list-style-type: lower-alpha;">\2</ol>',
      'Ordered list alphabetically',
      '[list=a][*]item 1[*] item2[/list]',
      :list],
    'Definition List' => [
      /\[dl\](.*?)\[\/dl\]/im,
      '<dl>\1</dl>',
      'List of terms/items and their definitions',
      '[dl][dt]Fusion Reactor[/dt][dd]Chamber that provides power to your... nerd stuff[/dd][dt]Mass Cannon[/dt][dd]A gun of some sort[/dd][/dl]',
      :definelist],
    'Definition Term' => [
      /\[dt\](.*?)\[\/dt\]/mi,
      '<dt>\1</dt>',
      'List of definition terms',
      nil, nil,
      :defineterm],
    'Definition Definition' => [
      /\[dd\](.*?)\[\/dd\]/mi,
      '<dd>\1</dd>',
      'Definition definitions',
      nil, nil,
      :definition],
    'Quote' => [
      /\[quote(:.*)?="?(.*?)"?\](.*?)\[\/quote\1?\]/mi,
      '<fieldset><legend>\2</legend><blockquote>\3</blockquote></fieldset>',
      'Quote with citation',
      nil, nil,
      :quote],
    'Quote (Sourceless)' => [
      /\[quote(:.*)?\](.*?)\[\/quote\1?\]/mi,
      '<fieldset><blockquote>\2</blockquote></fieldset>',
      'Quote (sourceclass)',
      nil, nil,
      :quote],
    'Link' => [
      /\[url=(.*?)\](.*?)\[\/url\]/mi,
      '<a href="\1">\2</a>',
      'Hyperlink to somewhere else',
      'Maybe try looking on [url=http://google.com]Google[/url]?',
      nil, nil,
      :link],
    'Link (Implied)' => [
      /\[url\](.*?)\[\/url\]/mi,
      '<a href="\1">\1</a>',
      'Hyperlink (implied)',
      nil, nil,
      :link],
    'Link (Automatic)' => [
        /\s(https?:\/\/.*?(?=(\s|$)))/,
        ' <a href="\1">\1</a>',
     nil, nil,
     :link],
    'Email' => [
      /\[email(:.+)?\](.+)\[\/email\1?\]/i,
      '<a href="mailto:\2">\2</a>',
      'Link to email address',
      '[email]wadus@wadus.com[/email]'
    ]
  }

  def self.to_html(text, tags_alternative_definition = {}, method = :disable, escape_html = true, paragraphs = true, tags = [])
    text = text.clone
    # escape < and > to remove any html
    if escape_html
      text.gsub!( '<', '&lt;' )
      text.gsub!( '>', '&gt;' )
    end

    tags_definition = @@tags.merge(tags_alternative_definition)

    # parse bbcode tags
    case method
      when :enable
        tags_definition.each_value { |t|
          text.gsub!(t[0], t[1]) if tags.include?(t[4])
        }
      when :disable
        # this works nicely because the default is disable and the default set of tags is [] (so none disabled) :)
        tags_definition.each_value { |t|
          unless tags.include?(t[4])
            text.gsub!(t[0], t[1])
          end
        }
    end

    # parse spacing
    text.gsub!( /\r\n?/, "\n" )
    text.strip!
    text.gsub!(/\n\n/, "</p><p>") if paragraphs
    text.gsub!( /\n/, "<br />" )

    # return markup
    if paragraphs
      '<p>' + text + '</p>'
    else
      text
    end
  end

  def self.tags
    @@tags.each { |tn, ti|
      # yields the tag name, a description of it and example
      yield tn, ti[2], ti[3] if ti[2]
    }
  end

  def self.tag_list
    @@tags
  end
end

class String
  def bbcode_to_html(tags_alternative_definition = {}, method = :disable, escape_html = true, paragraphs = true, tags = [])
    BBRuby.to_html(self, tags_alternative_definition, method, escape_html, paragraphs, tags)
  end
  def bbcode_to_html!(tags_alternative_definition = {}, method = :disable, escape_html = true, paragraphs = true, tags = [])
    self.replace(BBRuby.to_html(self, tags_alternative_definition, method, escape_html, paragraphs, tags))
  end
end

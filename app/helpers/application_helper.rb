# Methods added to this helper will be available to all templates in the application.
module ApplicationHelper
  #Trim the text to no more than max_length characters, including ellipses if appropriate
  def trim_if_needed(text, max_length)
    if text.length > max_length
      text[0..max_length - 4] + '...'
    else
      text
    end
  end
end

ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:long] = '%A %d %B %Y at %I:%M %P %Z'
ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:short] = '%Y-%m-%d, %I:%M %P'
ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:default] = '%A %d %B %Y at %I:%M %P'

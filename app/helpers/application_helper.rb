# Methods added to this helper will be available to all templates in the application.
module ApplicationHelper
end

ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:long] = '%A %d %B %Y at %I:%M %P %Z'
ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:short] = '%Y-%m-%d, %I:%M %P'
ActiveSupport::CoreExtensions::Time::Conversions::DATE_FORMATS[:default] = '%A %d %B %Y at %I:%M %P'

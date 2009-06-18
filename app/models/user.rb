class User < ActiveRecord::Base
  validates_length_of :username, :minimum => 3, :allow_nil => true
  validates_uniqueness_of :username, :case_sensitive => false, :allow_nil => true
  validates_format_of :username, :with => Authentication.login_regex, :message => Authentication.bad_login_message, :allow_nil => true

  validates_uniqueness_of :email_address, :case_sensitive => false, :allow_nil => true

  validates_length_of :openid, :minimum => 7, :allow_nil => true
  validates_uniqueness_of :openid, :case_sensitive => false, :allow_nil => true

  validates_length_of :fullname, :minimum => 5
  validates_uniqueness_of :fullname, :case_sensitive => false
  validates_format_of :fullname, :with => Authentication.name_regex, :message => Authentication.bad_name_message

  has_many :said_quotes, :class_name => "Quote", :foreign_key => "quotee_id", :order => 'created_at DESC'
  has_and_belongs_to_many :contexts, :uniq => true

  include Authentication
  include Authentication::ByCookieToken


  validates_length_of       :fullname,     :maximum => 100

  # HACK HACK HACK -- how to do attr_accessible from here?
  # prevents a user from submitting a crafted form that bypasses activation
  # anything else you want your user to change should be added here.
  attr_accessible :username, :fullname, :email_address, :time_zone, :email_notification

  def openid=(value)
    write_attribute :openid, (value ? OpenIdAuthentication.normalize_identifier(value) : nil)
  end

  def email #Used by Gravatar plugin
    email_address || ''
  end

  #Find user with username or fullname matching the given string, or if none then find a list of possible matches
  #returns user (nil if no exact match), and possible matches (nil if user matched or string was nil or empty)
  def self.find_from_string(name_string, current_user)
    return [nil, nil] if name_string.nil? || name_string.empty?

    user = User.first(:conditions => ['username ILIKE ?', name_string])
    user = User.first(:conditions => ['fullname ILIKE ?', name_string]) if user.nil?

    if user.nil?
      #Find possible matches
      [nil, User.all(:conditions => ["username ILIKE '%' || ? || '%' OR fullname ILIKE '%' || ? || '%'", name_string, name_string], :order => "(SELECT COUNT(*) FROM quotes WHERE quotee_id = users.id AND quoter_id = #{current_user.id}) DESC", :limit => 10)]
    else
      [user, nil]
    end
  end

  alias_method :ar_to_xml, :to_xml

  def to_xml(options = {}, &block)
    default_options = { :except => [ :remember_token, :remember_token_expires_at, :email_address ]}
    self.ar_to_xml(options.merge(default_options), &block)
  end
end

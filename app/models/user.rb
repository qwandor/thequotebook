class User < ActiveRecord::Base
  validates_length_of :username, :minimum => 3
  validates_uniqueness_of :username, :case_sensitive => false
  validates_uniqueness_of :email_address, :case_sensitive => false
  has_many :said_quotes, :class_name => "Quote", :foreign_key => "quotee_id"

  include Authentication
  include Authentication::ByCookieToken

  validates_format_of       :username,    :with => Authentication.login_regex, :message => Authentication.bad_login_message

  validates_format_of       :fullname,     :with => Authentication.name_regex,  :message => Authentication.bad_name_message, :allow_nil => true
  validates_length_of       :fullname,     :maximum => 100

  # HACK HACK HACK -- how to do attr_accessible from here?
  # prevents a user from submitting a crafted form that bypasses activation
  # anything else you want your user to change should be added here.
  attr_accessible :username, :fullname, :email_address

  def openid=(value)
    write_attribute :openid, (value ? OpenIdAuthentication.normalize_identifier(value) : nil)
  end

  def email #Used by Gravatar plugin
    email_address || ''
  end
end

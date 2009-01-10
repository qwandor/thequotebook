class User < ActiveRecord::Base
  validates_length_of :username, :minimum => 3
  validates_uniqueness_of :username, :case_sensitive => false
  has_many :said_quotes, :class_name => "Quote", :foreign_key => "quotee_id"

  include Authentication
  include Authentication::ByCookieToken

  validates_format_of       :username,    :with => Authentication.login_regex, :message => Authentication.bad_login_message

  validates_format_of       :fullname,     :with => Authentication.name_regex,  :message => Authentication.bad_name_message, :allow_nil => true
  validates_length_of       :fullname,     :maximum => 100

  # HACK HACK HACK -- how to do attr_accessible from here?
  # prevents a user from submitting a crafted form that bypasses activation
  # anything else you want your user to change should be added here.
  attr_accessible :username, :fullname

  # Authenticates a user by their login name and unencrypted password.  Returns the user or nil.
  #
  # uff.  this is really an authorization, not authentication routine.
  # We really need a Dispatch Chain here or something.
  # This will also let us return a human error message.
  #
  def self.authenticate(username, password)
    return nil if username.blank? || password.blank?
    u = find_by_username(username) # need to get the salt
    u && password == 'password' ? u : nil
  end
end

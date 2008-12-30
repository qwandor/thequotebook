require File.dirname(__FILE__) + '/../test_helper'

class UsexTest < ActiveSupport::TestCase
  # Be sure to include AuthenticatedTestHelper in test/test_helper.rb instead.
  # Then, you can remove it from this and the functional test.
  include AuthenticatedTestHelper
  fixtures :usexes

  def test_should_create_usex
    assert_difference 'Usex.count' do
      usex = create_usex
      assert !usex.new_record?, "#{usex.errors.full_messages.to_sentence}"
    end
  end

  def test_should_require_login
    assert_no_difference 'Usex.count' do
      u = create_usex(:login => nil)
      assert u.errors.on(:login)
    end
  end

  def test_should_require_password
    assert_no_difference 'Usex.count' do
      u = create_usex(:password => nil)
      assert u.errors.on(:password)
    end
  end

  def test_should_require_password_confirmation
    assert_no_difference 'Usex.count' do
      u = create_usex(:password_confirmation => nil)
      assert u.errors.on(:password_confirmation)
    end
  end

  def test_should_require_email
    assert_no_difference 'Usex.count' do
      u = create_usex(:email => nil)
      assert u.errors.on(:email)
    end
  end

  def test_should_reset_password
    usexes(:quentin).update_attributes(:password => 'new password', :password_confirmation => 'new password')
    assert_equal usexes(:quentin), Usex.authenticate('quentin', 'new password')
  end

  def test_should_not_rehash_password
    usexes(:quentin).update_attributes(:login => 'quentin2')
    assert_equal usexes(:quentin), Usex.authenticate('quentin2', 'monkey')
  end

  def test_should_authenticate_usex
    assert_equal usexes(:quentin), Usex.authenticate('quentin', 'monkey')
  end

  def test_should_set_remember_token
    usexes(:quentin).remember_me
    assert_not_nil usexes(:quentin).remember_token
    assert_not_nil usexes(:quentin).remember_token_expires_at
  end

  def test_should_unset_remember_token
    usexes(:quentin).remember_me
    assert_not_nil usexes(:quentin).remember_token
    usexes(:quentin).forget_me
    assert_nil usexes(:quentin).remember_token
  end

  def test_should_remember_me_for_one_week
    before = 1.week.from_now.utc
    usexes(:quentin).remember_me_for 1.week
    after = 1.week.from_now.utc
    assert_not_nil usexes(:quentin).remember_token
    assert_not_nil usexes(:quentin).remember_token_expires_at
    assert usexes(:quentin).remember_token_expires_at.between?(before, after)
  end

  def test_should_remember_me_until_one_week
    time = 1.week.from_now.utc
    usexes(:quentin).remember_me_until time
    assert_not_nil usexes(:quentin).remember_token
    assert_not_nil usexes(:quentin).remember_token_expires_at
    assert_equal usexes(:quentin).remember_token_expires_at, time
  end

  def test_should_remember_me_default_two_weeks
    before = 2.weeks.from_now.utc
    usexes(:quentin).remember_me
    after = 2.weeks.from_now.utc
    assert_not_nil usexes(:quentin).remember_token
    assert_not_nil usexes(:quentin).remember_token_expires_at
    assert usexes(:quentin).remember_token_expires_at.between?(before, after)
  end

protected
  def create_usex(options = {})
    record = Usex.new({ :login => 'quire', :email => 'quire@example.com', :password => 'quire69', :password_confirmation => 'quire69' }.merge(options))
    record.save
    record
  end
end

require File.dirname(__FILE__) + '/../test_helper'
require 'usexes_controller'

# Re-raise errors caught by the controller.
class UsexesController; def rescue_action(e) raise e end; end

class UsexesControllerTest < ActionController::TestCase
  # Be sure to include AuthenticatedTestHelper in test/test_helper.rb instead
  # Then, you can remove it from this and the units test.
  include AuthenticatedTestHelper

  fixtures :usexes

  def test_should_allow_signup
    assert_difference 'Usex.count' do
      create_usex
      assert_response :redirect
    end
  end

  def test_should_require_login_on_signup
    assert_no_difference 'Usex.count' do
      create_usex(:login => nil)
      assert assigns(:usex).errors.on(:login)
      assert_response :success
    end
  end

  def test_should_require_password_on_signup
    assert_no_difference 'Usex.count' do
      create_usex(:password => nil)
      assert assigns(:usex).errors.on(:password)
      assert_response :success
    end
  end

  def test_should_require_password_confirmation_on_signup
    assert_no_difference 'Usex.count' do
      create_usex(:password_confirmation => nil)
      assert assigns(:usex).errors.on(:password_confirmation)
      assert_response :success
    end
  end

  def test_should_require_email_on_signup
    assert_no_difference 'Usex.count' do
      create_usex(:email => nil)
      assert assigns(:usex).errors.on(:email)
      assert_response :success
    end
  end
  

  

  protected
    def create_usex(options = {})
      post :create, :usex => { :login => 'quire', :email => 'quire@example.com',
        :password => 'quire69', :password_confirmation => 'quire69' }.merge(options)
    end
end

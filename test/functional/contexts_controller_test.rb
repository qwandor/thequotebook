require 'test_helper'

class ContextsControllerTest < ActionController::TestCase
  def test_should_get_index
    get :index
    assert_response :success
    assert_not_nil assigns(:contexts)
  end

  def test_should_get_new
    get :new
    assert_response :success
  end

  def test_should_create_context
    assert_difference('Context.count') do
      post :create, :context => { }
    end

    assert_redirected_to context_path(assigns(:context))
  end

  def test_should_show_context
    get :show, :id => contexts(:one).id
    assert_response :success
  end

  def test_should_get_edit
    get :edit, :id => contexts(:one).id
    assert_response :success
  end

  def test_should_update_context
    put :update, :id => contexts(:one).id, :context => { }
    assert_redirected_to context_path(assigns(:context))
  end

  def test_should_destroy_context
    assert_difference('Context.count', -1) do
      delete :destroy, :id => contexts(:one).id
    end

    assert_redirected_to contexts_path
  end
end

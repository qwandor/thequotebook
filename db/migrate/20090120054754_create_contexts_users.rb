class CreateContextsUsers < ActiveRecord::Migration
  def self.up
    create_table :contexts_users, :id => false do |t|
      t.references :context, :null => false
      t.references :user, :null => false
    end
    add_index :contexts_users, [:context_id]
    add_index :contexts_users, [:user_id]
    add_index :contexts_users, [:context_id, :user_id], :unique => true
  end

  def self.down
    drop_table :contexts_users
  end
end

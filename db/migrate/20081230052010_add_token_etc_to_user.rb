class AddTokenEtcToUser < ActiveRecord::Migration
  def self.up
    add_column :users, :remember_token, :string, :limit => 40
    add_column :users, :remember_token_expires_at, :datetime
    add_index :users, :username, :unique => true
  end

  def self.down
    remove_column :users, :remember_token_expires_at
    remove_column :users, :remember_token
    remove_index :users, :username
  end
end

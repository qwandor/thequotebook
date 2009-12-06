class AddHiddenToQuote < ActiveRecord::Migration
  def self.up
    add_column :quotes, :hidden, :boolean, :default => false, :null => false
  end

  def self.down
    remove_column :quotes, :hidden
  end
end

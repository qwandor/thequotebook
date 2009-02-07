class CreateComments < ActiveRecord::Migration
  def self.up
    create_table :comments do |t|
      t.references :quote, :null => false
      t.references :user, :null => false
      t.text :body, :null => false

      t.timestamps
    end
    add_index :comments, [:quote_id]
  end

  def self.down
    drop_table :comments
  end
end

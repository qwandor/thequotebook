class CreateQuotes < ActiveRecord::Migration
  def self.up
    create_table :quotes do |t|
      t.text :quote_text
      t.references :context
      t.references :quoter
      t.references :quotee

      t.timestamps
    end
  end

  def self.down
    drop_table :quotes
  end
end

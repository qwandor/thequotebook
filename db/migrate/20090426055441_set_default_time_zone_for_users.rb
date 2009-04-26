class SetDefaultTimeZoneForUsers < ActiveRecord::Migration
  def self.up
    change_column :users, :time_zone, :string, :default => 'Wellington'
  end

  def self.down
    change_column :users, :time_zone, :string
  end
end

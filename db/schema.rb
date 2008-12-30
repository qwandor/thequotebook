# This file is auto-generated from the current state of the database. Instead of editing this file, 
# please use the migrations feature of Active Record to incrementally modify your database, and
# then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your database schema. If you need
# to create the application database on another system, you should be using db:schema:load, not running
# all the migrations from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended to check this file into your version control system.

ActiveRecord::Schema.define(:version => 20081230052010) do

  create_table "contexts", :force => true do |t|
    t.string   "name"
    t.string   "description"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "quotes", :force => true do |t|
    t.text     "quote_text"
    t.integer  "context_id"
    t.integer  "quoter_id",  :null => false
    t.integer  "quotee_id",  :null => false
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "users", :force => true do |t|
    t.string   "username"
    t.string   "fullname"
    t.string   "openid"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.string   "remember_token",            :limit => 40
    t.datetime "remember_token_expires_at"
  end

  add_index "users", ["username"], :name => "index_users_on_username", :unique => true

  add_foreign_key "quotes", ["context_id"], "contexts", ["id"], :name => "quotes_context_id_fkey"
  add_foreign_key "quotes", ["quoter_id"], "users", ["id"], :name => "quotes_quoter_id_fkey"
  add_foreign_key "quotes", ["quotee_id"], "users", ["id"], :name => "quotes_quotee_id_fkey"

end

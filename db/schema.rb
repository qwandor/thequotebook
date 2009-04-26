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

ActiveRecord::Schema.define(:version => 20090426044417) do

  create_table "comments", :force => true do |t|
    t.integer  "quote_id",   :null => false
    t.integer  "user_id",    :null => false
    t.text     "body",       :null => false
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  add_index "comments", ["quote_id"], :name => "index_comments_on_quote_id"

  create_table "contexts", :force => true do |t|
    t.string   "name"
    t.string   "description"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "contexts_users", :id => false, :force => true do |t|
    t.integer "context_id", :null => false
    t.integer "user_id",    :null => false
  end

  add_index "contexts_users", ["context_id"], :name => "index_contexts_users_on_context_id"
  add_index "contexts_users", ["context_id", "user_id"], :name => "index_contexts_users_on_context_id_and_user_id", :unique => true
  add_index "contexts_users", ["user_id"], :name => "index_contexts_users_on_user_id"

  create_table "open_id_authentication_associations", :force => true do |t|
    t.integer "issued"
    t.integer "lifetime"
    t.string  "handle"
    t.string  "assoc_type"
    t.binary  "server_url"
    t.binary  "secret"
  end

  create_table "open_id_authentication_nonces", :force => true do |t|
    t.integer "timestamp",  :null => false
    t.string  "server_url"
    t.string  "salt",       :null => false
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
    t.string   "email_address"
    t.string   "time_zone"
  end

  add_index "users", ["username"], :name => "index_users_on_username", :unique => true

  add_foreign_key "comments", ["quote_id"], "quotes", ["id"], :name => "comments_quote_id_fkey"
  add_foreign_key "comments", ["user_id"], "users", ["id"], :name => "comments_user_id_fkey"

  add_foreign_key "contexts_users", ["context_id"], "contexts", ["id"], :name => "contexts_users_context_id_fkey"
  add_foreign_key "contexts_users", ["user_id"], "users", ["id"], :name => "contexts_users_user_id_fkey"

  add_foreign_key "quotes", ["context_id"], "contexts", ["id"], :name => "quotes_context_id_fkey"
  add_foreign_key "quotes", ["quoter_id"], "users", ["id"], :name => "quotes_quoter_id_fkey"
  add_foreign_key "quotes", ["quotee_id"], "users", ["id"], :name => "quotes_quotee_id_fkey"

end

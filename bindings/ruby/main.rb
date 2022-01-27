require 'ffi'
require 'json'
require "os"

module Templating
  extend FFI::Library
  ffi_lib '../target/debug/librs_templating.so'
  attach_function :variables, [ :string ], :pointer
  attach_function :execute, [ :string, :string ], :pointer
  attach_function :release, [ :pointer ], :pointer
end

puts "#{OS.rss_bytes / 1_000_000} MB"

n = 1
(1..n).each do |i|
  tpl = "Hello, {{ name }}. {{ greet | title }}"
  ptr = Templating.variables(tpl)
  puts ptr.read_string()
  Templating.release(ptr)

  ptr = Templating.execute(tpl, {name: "sir", greet: "what DO you think???"}.to_json)
  puts ptr.read_string()
  Templating.release(ptr)
end

puts "#{OS.rss_bytes / 1_000_000} MB"

require "./value"
require "./ast"
require "./lexer"
require "./parser"
require "./builtins"
require "./interp"

def main
  args = ARGV

  if args.size < 1
    STDERR.puts "usage: awk '{print}' file.txt"
    exit(1)
  end

  program = args[0]
  files = args.size > 1 ? args[1..] : [] of String

  lexer = Lexer.new(program)
  parser = Parser.new(lexer)
  node = parser.parse

  interp = Interp.new

  b_out = interp.exec_begin(node)
  print b_out

  if !files.empty?
    files.each do |file|
      interp.env.set_filename(file)

      bf_out2 = interp.exec_begin_file(node)
      print bf_out2

      content = File.read(file)
      interp.in_range = false
      content.each_line do |line|
        line_out = interp.exec_line(node, line)
        print line_out
      end

      ef_out2 = interp.exec_end_file(node)
      print ef_out2
    end
  else
    interp.env.set_filename("")
    STDIN.each_line do |line|
      line_out = interp.exec_line(node, line)
      print line_out
    end
  end

  e_out = interp.exec_end(node)
  print e_out
end

main

require "./value"
require "./ast"
require "./lexer"
require "./parser"
require "./builtins"
require "./interp"

def main
  args = ARGV.dup
  fs = " "

  # Parse -F flag
  prog_idx = 0
  i = 0
  while i < args.size
    if args[i] == "-F" && i + 1 < args.size
      fs = args[i + 1]
      args.delete_at(i)
      args.delete_at(i)
    elsif args[i].starts_with?("-F")
      fs = args[i][2..]
      args.delete_at(i)
    elsif args[i] == "-f" && i + 1 < args.size
      prog_file = args[i + 1]
      args.delete_at(i)
      args.delete_at(i)
      args.unshift(File.read(prog_file).strip)
      prog_idx = 0
    elsif args[i] == "-v" && i + 1 < args.size
      var_def = args[i + 1]
      args.delete_at(i)
      args.delete_at(i)
      args.unshift("BEGIN{#{var_def}}") unless var_def.empty?
      i += 1
    else
      i += 1
    end
  end

  if args.size < 1
    STDERR.puts "usage: awk '{print}' file.txt"
    exit(1)
  end

  program = args[0]
  files = args.size > 1 ? args[1..] : [] of String

  lexer = Lexer.new(program)
  parser = Parser.new(lexer)
  node = begin
    parser.parse
  rescue ex
    STDERR.puts "syntax error: #{ex.message}"
    exit(1)
  end

  interp = Interp.new
  interp.env.fs = fs

  b_out, b_signal = interp.exec_begin(node)
  print b_out
  if b_signal == ControlFlow::Exit
    exit(interp.exit_code)
  end

  line_loop = ->(line : String) {
    line = line.chomp
    result, signal = interp.exec_line(node, line, interp.env.fs)
    print result
    if signal == ControlFlow::Exit
      exit(interp.exit_code)
    end
    signal
  }

  if !files.empty?
    files.each do |file|
      interp.env.set_filename(file)

      bf_out, bf_signal = interp.exec_begin_file(node)
      print bf_out
      if bf_signal == ControlFlow::Exit
        exit(interp.exit_code)
      end

      content = File.read(file)
      interp.in_range = false
      content.each_line do |line|
        signal = line_loop.call(line)
        break if signal == ControlFlow::Exit
      end

      ef_out, ef_signal = interp.exec_end_file(node)
      print ef_out
      if ef_signal == ControlFlow::Exit
        exit(interp.exit_code)
      end
    end
  elsif node.needs_input?
    interp.env.set_filename("")
    STDIN.each_line do |line|
      line = line.chomp
      result, signal = interp.exec_line(node, line, interp.env.fs)
      print result
      if signal == ControlFlow::Exit
        exit(interp.exit_code)
      end
    end
  end

  e_out, e_signal = interp.exec_end(node)
  print e_out
  if e_signal == ControlFlow::Exit
    exit(interp.exit_code)
  end
end

main

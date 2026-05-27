enum ControlFlow
  Normal
  Break
  Continue
  Next
  Exit
end

class Interp
  getter env : Env
  property in_range : Bool
  getter exit_code : Int32
  @regex_cache : Regex

  def initialize
    @env = Env.new
    @in_range = false
    @regex_cache = Regex.new("")
    @exit_code = 0
  end

  def exec_begin(node : Program) : {String, ControlFlow}
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(Begin)
        signal = exec_stmts(rule.stmts, buf)
        if signal != ControlFlow::Normal
          return {buf.to_s, signal}
        end
      end
    end
    {buf.to_s, ControlFlow::Normal}
  end

  def exec_begin_file(node : Program) : {String, ControlFlow}
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(BeginFile)
        signal = exec_stmts(rule.stmts, buf)
        if signal != ControlFlow::Normal
          return {buf.to_s, signal}
        end
      end
    end
    {buf.to_s, ControlFlow::Normal}
  end

  def exec_end_file(node : Program) : {String, ControlFlow}
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(EndFile)
        signal = exec_stmts(rule.stmts, buf)
        if signal != ControlFlow::Normal
          return {buf.to_s, signal}
        end
      end
    end
    {buf.to_s, ControlFlow::Normal}
  end

  def exec_end(node : Program) : {String, ControlFlow}
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(End)
        signal = exec_stmts(rule.stmts, buf)
        if signal != ControlFlow::Normal
          return {buf.to_s, signal}
        end
      end
    end
    {buf.to_s, ControlFlow::Normal}
  end

  def exec_line(node : Program, line : String, field_sep : String = " ") : {String, ControlFlow}
    @env.set_line(line, field_sep)
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(PatternRule)
        if matches_pattern(rule.pattern)
          signal = exec_stmts(rule.stmts, buf)
          if signal != ControlFlow::Normal
            return {buf.to_s, signal}
          end
        end
      end
    end
    {buf.to_s, ControlFlow::Normal}
  end

  private def exec_stmts(stmts : Array(Stmt), buf : String::Builder) : ControlFlow
    stmts.each do |stmt|
      signal = exec_stmt(stmt, buf)
      if signal != ControlFlow::Normal
        return signal
      end
    end
    ControlFlow::Normal
  end

  private def exec_stmt(stmt : Stmt, buf : String::Builder) : ControlFlow
    case stmt
    when PrintStmt
      exec_print(stmt.exprs, buf)
      ControlFlow::Normal
    when PrintfStmt
      exec_printf(stmt.fmt, stmt.args, buf)
      ControlFlow::Normal
    when ExprStmt
      eval_expr(stmt.expr)
      ControlFlow::Normal
    when IfStmt
      exec_if(stmt, buf)
    when WhileStmt
      exec_while(stmt, buf)
    when ForStmt
      exec_for(stmt, buf)
    when ForInStmt
      exec_for_in(stmt, buf)
    when BreakStmt
      ControlFlow::Break
    when ContinueStmt
      ControlFlow::Continue
    when NextStmt
      ControlFlow::Next
    when ExitStmt
      if e = stmt.expr
        @exit_code = eval_expr(e).as_f64.to_i32
      end
      ControlFlow::Exit
    when DeleteStmt
      exec_delete(stmt)
      ControlFlow::Normal
    else
      ControlFlow::Normal
    end
  end

  private def exec_print(exprs : Array(Expr), buf : String::Builder)
    if exprs.empty?
      buf << @env.get_field(0_u64).to_s << @env.ors
      return
    end
    parts = exprs.map { |e| eval_expr(e).to_s }
    parts.each_with_index do |part, i|
      buf << @env.ofs if i > 0
      buf << part
    end
    buf << @env.ors
  end

  private def exec_printf(fmt : Expr, args : Array(Expr), buf : String::Builder)
    fmt_str = eval_expr(fmt).to_s
    arg_vals = args.map { |a| eval_expr(a) }
    process_format(fmt_str, arg_vals, buf)
  end

  private def process_format(fmt : String, args : Array(AwkValue), buf : String::Builder)
    i = 0
    arg_idx = 0
    while i < fmt.size
      if fmt[i] == '%' && i + 1 < fmt.size
        i += 1
        flags = ""
        width = 0
        precision = -1
        if fmt[i] == '-'
          flags = "-"
          i += 1
        end
        while i < fmt.size && fmt[i] >= '0' && fmt[i] <= '9'
          width = width * 10 + (fmt[i] - '0').to_i32
          i += 1
        end
        if i < fmt.size && fmt[i] == '.'
          i += 1
          precision = 0
          while i < fmt.size && fmt[i] >= '0' && fmt[i] <= '9'
            precision = precision * 10 + (fmt[i] - '0').to_i32
            i += 1
          end
        end
        if i < fmt.size
          case fmt[i]
          when '%'
            buf << '%'
          when 's'
            val = arg_idx < args.size ? args[arg_idx].to_s : ""
            arg_idx += 1
            apply_format(val, width, flags, buf)
          when 'd'
            val = arg_idx < args.size ? args[arg_idx].as_f64.to_i32.to_s : "0"
            arg_idx += 1
            apply_format(val, width, flags, buf)
          when 'f'
            val = arg_idx < args.size ? args[arg_idx].as_f64 : 0.0
            arg_idx += 1
            prec = precision >= 0 ? precision : 6
            s = sprintf("%.#{prec}f", val)
            apply_format(s, width, flags, buf)
          when 'e'
            val = arg_idx < args.size ? args[arg_idx].as_f64 : 0.0
            arg_idx += 1
            prec = precision >= 0 ? precision : 6
            s = sprintf("%.#{prec}e", val)
            apply_format(s, width, flags, buf)
          when 'g'
            val = arg_idx < args.size ? args[arg_idx].as_f64 : 0.0
            arg_idx += 1
            prec = precision >= 0 ? precision : 6
            s = sprintf("%.#{prec}g", val)
            apply_format(s, width, flags, buf)
          else
            buf << '%' << fmt[i]
          end
          i += 1
        end
      else
        buf << fmt[i]
        i += 1
      end
    end
  end

  private def apply_format(val : String, width : Int32, flags : String, buf : String::Builder)
    if width > 0 && val.size < width
      padding = " " * (width - val.size)
      if flags == "-"
        buf << val << padding
      else
        buf << padding << val
      end
    else
      buf << val
    end
  end

  private def exec_if(stmt : IfStmt, buf : String::Builder) : ControlFlow
    cv = eval_expr(stmt.cond)
    if is_truthy(cv)
      exec_stmts(stmt.then_branch, buf)
    elsif eb = stmt.else_branch
      exec_stmts(eb, buf)
    else
      ControlFlow::Normal
    end
  end

  private def exec_while(stmt : WhileStmt, buf : String::Builder) : ControlFlow
    while is_truthy(eval_expr(stmt.cond))
      signal = exec_stmts(stmt.body, buf)
      case signal
      when ControlFlow::Break  then break
      when ControlFlow::Continue then next
      when ControlFlow::Next   then return ControlFlow::Next
      when ControlFlow::Exit   then return ControlFlow::Exit
      end
    end
    ControlFlow::Normal
  end

  private def exec_for(stmt : ForStmt, buf : String::Builder) : ControlFlow
    if init = stmt.init
      exec_stmt(init, buf)
    end
    while true
      if cond = stmt.cond
        break unless is_truthy(eval_expr(cond))
      end
      signal = exec_stmts(stmt.body, buf)
      if signal == ControlFlow::Break
        break
      elsif signal == ControlFlow::Next
        return ControlFlow::Next
      elsif signal == ControlFlow::Exit
        return ControlFlow::Exit
      end
      if update = stmt.update
        eval_expr(update)
      end
    end
    ControlFlow::Normal
  end

  private def exec_for_in(stmt : ForInStmt, buf : String::Builder) : ControlFlow
    keys = @env.array_keys(stmt.array_name)
    keys.each do |key|
      @env.set_var(stmt.var, AwkValue.new(key))
      signal = exec_stmts(stmt.body, buf)
      case signal
      when ControlFlow::Break  then break
      when ControlFlow::Continue then next
      when ControlFlow::Next   then return ControlFlow::Next
      when ControlFlow::Exit   then return ControlFlow::Exit
      end
    end
    ControlFlow::Normal
  end

  private def exec_delete(stmt : DeleteStmt)
    if idx = stmt.index
      @env.array_delete(stmt.name, eval_expr(idx))
    else
      @env.array_delete(stmt.name, nil)
    end
  end

  private def matches_pattern(pattern : Pattern) : Bool
    case pattern
    when ExprPattern
      v = eval_expr(pattern.expr)
      is_truthy(v)
    when RangePattern
      if @in_range
        v = eval_expr(pattern.end_expr)
        @in_range = false if is_truthy(v)
        true
      else
        v = eval_expr(pattern.start)
        if is_truthy(v)
          @in_range = true
          true
        else
          false
        end
      end
    else
      false
    end
  end

  private def is_truthy(val : AwkValue) : Bool
    val.truthy?
  end

  private def eval_expr(expr : Expr) : AwkValue
    case expr
    when FieldExpr
      @env.get_field(expr.index)
    when FieldDynExpr
      v = eval_expr(expr.inner)
      n = v.as_f64.to_i32
      @env.get_field(n)
    when IdentExpr
      @env.get_var(expr.name)
    when StrExpr
      AwkValue.new(expr.value)
    when NumExpr
      AwkValue.new(expr.value)
    when BinOpExpr
      eval_binop(expr)
    when UnaryOpExpr
      eval_unaryop(expr)
    when TernaryExpr
      cv = eval_expr(expr.cond)
      if is_truthy(cv)
        eval_expr(expr.true_branch)
      else
        eval_expr(expr.false_branch)
      end
    when AssignExpr
      val = eval_expr(expr.value)
      @env.set_var(expr.name, val)
      val
    when AssignOpExpr
      curr = @env.get_var(expr.name)
      rhs = eval_expr(expr.value)
      result = arith_op(curr, expr.op, rhs)
      @env.set_var(expr.name, result)
      result
    when IncrExpr
      curr = @env.get_var(expr.name)
      delta = AwkValue.new(expr.delta.to_f64)
      if expr.is_post
        result = curr
        new_val = arith_op(curr, BinOpKind::Add, delta)
        @env.set_var(expr.name, new_val)
        result
      else
        new_val = arith_op(curr, BinOpKind::Add, delta)
        @env.set_var(expr.name, new_val)
        new_val
      end
    when CallExpr
      eval_call(expr.name, expr.args)
    when ArrayAccessExpr
      idx = eval_expr(expr.index)
      @env.array_get(expr.name, idx)
    when ArrayAssignExpr
      idx = eval_expr(expr.index)
      val = eval_expr(expr.value)
      @env.array_set(expr.name, idx, val)
      val
    when FieldAssignExpr
      val = eval_expr(expr.value)
      @env.set_field(expr.index.to_i32, val)
      val
    when FieldDynAssignExpr
      idx = eval_expr(expr.inner).as_f64.to_i32
      val = eval_expr(expr.value)
      @env.set_field(idx, val)
      val
    else
      AwkValue.new(0.0)
    end
  end

  private def arith_op(lhs : AwkValue, kind : BinOpKind, rhs : AwkValue) : AwkValue
    case kind
    when BinOpKind::Add
      AwkValue.new(lhs.as_f64 + rhs.as_f64)
    when BinOpKind::Sub
      AwkValue.new(lhs.as_f64 - rhs.as_f64)
    when BinOpKind::Mul
      AwkValue.new(lhs.as_f64 * rhs.as_f64)
    when BinOpKind::Div
      r = rhs.as_f64
      if r == 0.0
        AwkValue.new(0.0)
      else
        AwkValue.new(lhs.as_f64 / r)
      end
    when BinOpKind::Mod
      r = rhs.as_f64
      if r == 0.0
        AwkValue.new(0.0)
      else
        AwkValue.new(lhs.as_f64 % r)
      end
    when BinOpKind::Pow
      l = lhs.as_f64
      r = rhs.as_f64
      ri = r.to_i64
      if r == ri.to_f64 && ri.abs < 1000
        AwkValue.new(l ** ri)
      else
        AwkValue.new(l ** r)
      end
    else
      AwkValue.new(0.0)
    end
  end

  private def eval_call(name : String, args : Array(Expr)) : AwkValue
    arg_vals = args.map { |a| eval_expr(a) }
    case name
    when "length"
      s = arg_vals.size > 0 ? arg_vals[0].to_s : @env.get_field(0_u64).to_s
      AwkValue.new(s.size.to_f64)
    when "toupper"
      AwkValue.new(arg_vals[0].to_s.upcase)
    when "tolower"
      AwkValue.new(arg_vals[0].to_s.downcase)
    when "substr"
      s = arg_vals[0].to_s
      start = arg_vals[1].as_f64.to_i32 - 1
      start = 0 if start < 0
      if arg_vals.size > 2
        len = arg_vals[2].as_f64.to_i32
        len = 0 if len < 0
        AwkValue.new(s[start, len]? || "")
      else
        AwkValue.new(s[start..]? || "")
      end
    when "index"
      s = arg_vals[0].to_s
      t = arg_vals[1].to_s
      pos = s.index(t)
      AwkValue.new(pos ? (pos + 1).to_f64 : 0.0)
    when "split"
      s = arg_vals[0].to_s
      arr_name = ""
      sep = arg_vals.size > 2 ? arg_vals[2].to_s : @env.fs
      # args[1] is array name as identifier
      if args.size > 1 && args[1].is_a?(IdentExpr)
        arr_name = args[1].as(IdentExpr).name
      end
      parts = if sep == " "
                s.split(/\s+/)
              else
                s.split(sep)
              end
      parts.each_with_index do |part, i|
        @env.array_set(arr_name, AwkValue.new((i + 1).to_s), AwkValue.new(part))
      end
      AwkValue.new(parts.size.to_f64)
    when "sub"
      if arg_vals.size >= 2
        re_str = arg_vals[0].to_s
        repl_str = arg_vals[1].to_s
        target_expr = args.size > 2 ? args[2] : nil
        target_str = target_expr ? eval_expr(target_expr).to_s : @env.get_field(0_u64).to_s
        re = Regex.new(re_str)
        result = target_str.sub(re, repl_str)
        n = target_str != result ? 1 : 0
        if n == 1
          if target_expr.is_a?(IdentExpr)
            @env.set_var(target_expr.name, AwkValue.new(result))
          else
            @env.set_field(0, AwkValue.new(result))
          end
        end
        AwkValue.new(n.to_f64)
      else
        AwkValue.new(0.0)
      end
    when "gsub"
      if arg_vals.size >= 2
        re_str = arg_vals[0].to_s
        repl_str = arg_vals[1].to_s
        target_expr = args.size > 2 ? args[2] : nil
        target_str = target_expr ? eval_expr(target_expr).to_s : @env.get_field(0_u64).to_s
        re = Regex.new(re_str)
        count = 0
        result = target_str.gsub(re) { |m| count += 1; repl_str }
        if count > 0
          if target_expr.is_a?(IdentExpr)
            @env.set_var(target_expr.name, AwkValue.new(result))
          else
            @env.set_field(0, AwkValue.new(result))
          end
        end
        AwkValue.new(count.to_f64)
      else
        AwkValue.new(0.0)
      end
    when "match"
      if arg_vals.size >= 2
        s = arg_vals[0].to_s
        re = Regex.new(arg_vals[1].to_s)
        if md = s.match(re)
          @env.set_var("RSTART", AwkValue.new((md.begin(0) + 1).to_f64))
          @env.set_var("RLENGTH", AwkValue.new((md.end(0) - md.begin(0)).to_f64))
          AwkValue.new((md.begin(0) + 1).to_f64)
        else
          @env.set_var("RSTART", AwkValue.new(0.0))
          @env.set_var("RLENGTH", AwkValue.new(-1.0))
          AwkValue.new(0.0)
        end
      else
        AwkValue.new(0.0)
      end
    when "sprintf"
      fmt = arg_vals[0].to_s
      rest = arg_vals[1..]
      buf = String::Builder.new
      process_format(fmt, rest, buf)
      AwkValue.new(buf.to_s)
    when "int"
      AwkValue.new(arg_vals[0].as_f64.trunc.to_f64)
    when "sqrt"
      AwkValue.new(Math.sqrt(arg_vals[0].as_f64))
    when "sin"
      AwkValue.new(Math.sin(arg_vals[0].as_f64))
    when "cos"
      AwkValue.new(Math.cos(arg_vals[0].as_f64))
    when "exp"
      AwkValue.new(Math.exp(arg_vals[0].as_f64))
    when "log"
      AwkValue.new(Math.log(arg_vals[0].as_f64))
    when "rand"
      AwkValue.new(Random.rand)
    when "srand"
      AwkValue.new(0.0)
    else
      AwkValue.new(0.0)
    end
  end

  private def eval_binop(expr : BinOpExpr) : AwkValue
    case expr.kind
    when BinOpKind::Concat
      lv = eval_expr(expr.lhs)
      rv = eval_expr(expr.rhs)
      AwkValue.new(lv.to_s + rv.to_s)
    when BinOpKind::And
      lv = eval_expr(expr.lhs)
      return lv unless is_truthy(lv)
      eval_expr(expr.rhs)
    when BinOpKind::Or
      lv = eval_expr(expr.lhs)
      return lv if is_truthy(lv)
      eval_expr(expr.rhs)
    when BinOpKind::Match, BinOpKind::NotMatch
      text = eval_expr(expr.lhs).to_s
      re_str = eval_expr(expr.rhs).to_s
      matched = begin
        r = Regex.new(re_str)
        @regex_cache = r
        r === text
      rescue
        false
      end
      AwkValue.new((matched == (expr.kind == BinOpKind::Match)) ? 1.0 : 0.0)
    when BinOpKind::Eq, BinOpKind::Ne
      lv = eval_expr(expr.lhs)
      rv = eval_expr(expr.rhs)
      equal = lv == rv
      AwkValue.new((equal == (expr.kind == BinOpKind::Eq)) ? 1.0 : 0.0)
    when BinOpKind::Lt, BinOpKind::Gt, BinOpKind::Le, BinOpKind::Ge
      eval_compare(expr)
    when BinOpKind::Add, BinOpKind::Sub, BinOpKind::Mul, BinOpKind::Div, BinOpKind::Mod, BinOpKind::Pow
      lv = eval_expr(expr.lhs)
      rv = eval_expr(expr.rhs)
      arith_op(lv, expr.kind, rv)
    else
      AwkValue.new(0.0)
    end
  end

  private def eval_compare(expr : BinOpExpr) : AwkValue
    lv = eval_expr(expr.lhs)
    rv = eval_expr(expr.rhs)
    a = lv.raw
    b = rv.raw

    result = if a.is_a?(Float64) && b.is_a?(Float64)
               compare_result(a <=> b, expr.kind)
             elsif a.is_a?(String) && b.is_a?(String)
               compare_result(a <=> b, expr.kind)
             elsif a.is_a?(Float64) && b.is_a?(String)
               if bn = b.to_f64?
                 compare_result(a <=> bn, expr.kind)
               else
                 compare_result(lv.to_s <=> rv.to_s, expr.kind)
               end
             elsif a.is_a?(String) && b.is_a?(Float64)
               if an = a.to_f64?
                 compare_result(an <=> b, expr.kind)
               else
                 compare_result(lv.to_s <=> rv.to_s, expr.kind)
               end
             else
               false
             end
    AwkValue.new(result ? 1.0 : 0.0)
  end

  private def compare_result(cmp : Int32 | Nil, kind : BinOpKind) : Bool
    case kind
    when BinOpKind::Lt then cmp == -1
    when BinOpKind::Gt then cmp == 1
    when BinOpKind::Le then cmp == -1 || cmp == 0
    when BinOpKind::Ge then cmp == 1 || cmp == 0
    else                    false
    end
  end

  private def eval_unaryop(expr : UnaryOpExpr) : AwkValue
    case expr.kind
    when UnaryOpKind::Not
      v = eval_expr(expr.expr)
      AwkValue.new(is_truthy(v) ? 0.0 : 1.0)
    when UnaryOpKind::Neg
      v = eval_expr(expr.expr)
      AwkValue.new(-v.as_f64)
    else
      AwkValue.new(0.0)
    end
  end
end

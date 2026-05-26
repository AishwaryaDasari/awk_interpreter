class Interp
  getter env : Env
  property in_range : Bool
  @regex_cache : Regex

  def initialize
    @env = Env.new
    @in_range = false
    @regex_cache = Regex.new("")
  end

  def exec_begin(node : Program) : String
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(Begin)
        exec_stmts(rule.stmts, buf)
      end
    end
    buf.to_s
  end

  def exec_begin_file(node : Program) : String
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(BeginFile)
        exec_stmts(rule.stmts, buf)
      end
    end
    buf.to_s
  end

  def exec_end_file(node : Program) : String
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(EndFile)
        exec_stmts(rule.stmts, buf)
      end
    end
    buf.to_s
  end

  def exec_end(node : Program) : String
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(End)
        exec_stmts(rule.stmts, buf)
      end
    end
    buf.to_s
  end

  def exec_line(node : Program, line : String) : String
    @env.set_line(line, " ")
    buf = String::Builder.new
    node.rules.each do |rule|
      if rule.is_a?(PatternRule)
        if matches_pattern(rule.pattern)
          exec_stmts(rule.stmts, buf)
        end
      end
    end
    buf.to_s
  end

  private def exec_stmts(stmts : Array(Stmt), buf : String::Builder)
    stmts.each do |stmt|
      case stmt
      when PrintStmt
        exec_print(stmt.exprs, buf)
      end
    end
  end

  private def exec_print(exprs : Array(Expr), buf : String::Builder)
    if exprs.empty?
      buf << @env.get_field(0_u64).to_s << '\n'
      return
    end
    exprs.each_with_index do |e, i|
      buf << ' ' if i > 0
      buf << eval_expr(e).to_s
    end
    buf << '\n'
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
    else
      AwkValue.new(0.0)
    end
  end
end

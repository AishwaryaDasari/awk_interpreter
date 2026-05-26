class Parser
  @lexer : Lexer
  @curr : Token
  @auto_match_regex : Bool

  def initialize(@lexer)
    @curr = @lexer.next_token
    @auto_match_regex = true
  end

  private def can_start_expr? : Bool
    case @curr.kind
    when TokenKind::String, TokenKind::Number, TokenKind::LParen,
         TokenKind::Dollar, TokenKind::Ident, TokenKind::Regex,
         TokenKind::Not, TokenKind::Plus, TokenKind::Minus,
         TokenKind::Incr, TokenKind::Decr
      true
    else
      false
    end
  end

  private def advance
    @curr = @lexer.next_token
  end

  private def err_unexpected : String
    "unexpected token: #{@curr.kind}"
  end

  private def expect(tok_kind : TokenKind) : Nil
    if @curr.kind == tok_kind
      advance
    else
      raise err_unexpected
    end
  end

  def parse : Program
    rules = [] of Rule
    loop do
      case @curr.kind
      when TokenKind::EOF
        break
      when TokenKind::Begin
        advance
        rules << Begin.new(parse_block)
      when TokenKind::End
        advance
        rules << End.new(parse_block)
      when TokenKind::BeginFile
        advance
        rules << BeginFile.new(parse_block)
      when TokenKind::EndFile
        advance
        rules << EndFile.new(parse_block)
      when TokenKind::LBrace
        rules << PatternRule.new(ExprPattern.new(NumExpr.new(1.0)), parse_block)
      else
        start = parse_expr
        if @curr.kind == TokenKind::Comma
          advance
          end_expr = parse_expr
          rules << PatternRule.new(RangePattern.new(start, end_expr), parse_block)
        else
          rules << PatternRule.new(ExprPattern.new(start), parse_block)
        end
      end
    end
    Program.new(rules)
  end

  private def parse_block : Array(Stmt)
    expect(TokenKind::LBrace)
    stmts = parse_stmts_until(TokenKind::RBrace)
    expect(TokenKind::RBrace)
    stmts
  end

  private def parse_stmts_until(end_kind : TokenKind) : Array(Stmt)
    stmts = [] of Stmt
    loop do
      case @curr.kind
      when end_kind, TokenKind::EOF
        break
      when TokenKind::Semicolon
        advance
      when TokenKind::RBrace
        break
      when TokenKind::Print
        stmts << parse_print
      when TokenKind::Printf
        stmts << parse_printf
      when TokenKind::If
        stmts << parse_if
      when TokenKind::While
        stmts << parse_while
      when TokenKind::For
        stmts << parse_for
      when TokenKind::Break
        advance
        stmts << BreakStmt.new
        if @curr.kind == TokenKind::Semicolon
          advance
        end
      when TokenKind::Continue
        advance
        stmts << ContinueStmt.new
        if @curr.kind == TokenKind::Semicolon
          advance
        end
      when TokenKind::Next
        advance
        stmts << NextStmt.new
        if @curr.kind == TokenKind::Semicolon
          advance
        end
      when TokenKind::Exit
        advance
        expr = @curr.kind == TokenKind::Semicolon || @curr.kind == end_kind ? nil : parse_expr
        stmts << ExitStmt.new(expr)
        if @curr.kind == TokenKind::Semicolon
          advance
        end
      when TokenKind::Delete
        stmts << parse_delete
      when TokenKind::LBrace
        stmts.concat(parse_block)
      else
        if can_start_expr?
          expr = parse_expr
          stmts << ExprStmt.new(expr)
          if @curr.kind == TokenKind::Semicolon
            advance
          end
        else
          raise err_unexpected
        end
      end
    end
    stmts
  end

  private def parse_print : Stmt
    advance
    exprs = [] of Expr
    loop do
      case @curr.kind
      when TokenKind::RBrace, TokenKind::EOF, TokenKind::Semicolon
        break
      when TokenKind::Comma
        advance
      else
        exprs << parse_expr
        if @curr.kind == TokenKind::Comma
          advance
        else
          break
        end
      end
    end
    PrintStmt.new(exprs)
  end

  private def parse_printf : Stmt
    advance
    has_parens = @curr.kind == TokenKind::LParen
    if has_parens
      advance
    end
    fmt = parse_expr
    args = [] of Expr
    while @curr.kind == TokenKind::Comma
      advance
      args << parse_expr
    end
    if has_parens
      expect(TokenKind::RParen)
    end
    PrintfStmt.new(fmt, args)
  end

  private def parse_if : Stmt
    advance
    expect(TokenKind::LParen)
    cond = parse_expr
    expect(TokenKind::RParen)
    then_branch = parse_single_stmt_or_block
    if @curr.kind == TokenKind::Semicolon
      advance
    end
    else_branch = nil
    if @curr.kind == TokenKind::Else
      advance
      if @curr.kind == TokenKind::If
        else_branch = [parse_if] of Stmt
      else
        else_branch = parse_single_stmt_or_block
      end
    end
    IfStmt.new(cond, then_branch, else_branch)
  end

  private def parse_while : Stmt
    advance
    expect(TokenKind::LParen)
    cond = parse_expr
    expect(TokenKind::RParen)
    body = parse_single_stmt_or_block
    WhileStmt.new(cond, body)
  end

  private def parse_for : Stmt
    advance
    expect(TokenKind::LParen)

    if @curr.kind == TokenKind::Ident && peek_for_in?
      var = @curr.str_value || ""
      advance
      expect(TokenKind::In)
      array_name = parse_expr
      if array_name.is_a?(IdentExpr)
        expect(TokenKind::RParen)
        body = parse_single_stmt_or_block
        return ForInStmt.new(var, array_name.name, body)
      else
        raise "expected array name after 'in'"
      end
    end

    init = nil
    if @curr.kind != TokenKind::Semicolon
      if can_start_expr?
        init_expr = parse_expr
        init = ExprStmt.new(init_expr)
      end
    end
    expect(TokenKind::Semicolon)

    cond = nil
    if @curr.kind != TokenKind::Semicolon
      if can_start_expr?
        cond = parse_expr
      end
    end
    expect(TokenKind::Semicolon)

    update = nil
    if @curr.kind != TokenKind::RParen
      if can_start_expr?
        update = parse_expr
      end
    end
    expect(TokenKind::RParen)

    body = parse_single_stmt_or_block
    ForStmt.new(init, cond, update, body)
  end

  private def peek_for_in? : Bool
    saved_pos = @lexer.pos
    saved_curr = @curr
    begin
      advance
      result = @curr.kind == TokenKind::In
      @lexer.pos = saved_pos
      @curr = saved_curr
      result
    rescue
      @lexer.pos = saved_pos
      @curr = saved_curr
      false
    end
  end

  private def parse_delete : Stmt
    advance
    name = @curr.str_value || ""
    expect(TokenKind::Ident)
    if @curr.kind == TokenKind::LBracket
      advance
      index = parse_expr
      expect(TokenKind::RBracket)
      DeleteStmt.new(name, index)
    else
      DeleteStmt.new(name)
    end
  end

  private def parse_single_stmt_or_block : Array(Stmt)
    if @curr.kind == TokenKind::LBrace
      parse_block
    else
      stmt = parse_single_stmt
      [stmt]
    end
  end

  private def parse_single_stmt : Stmt
    case @curr.kind
    when TokenKind::Print
      parse_print
    when TokenKind::Printf
      parse_printf
    when TokenKind::If
      parse_if
    when TokenKind::While
      parse_while
    when TokenKind::For
      parse_for
    when TokenKind::Break
      advance
      BreakStmt.new
    when TokenKind::Continue
      advance
      ContinueStmt.new
    when TokenKind::Next
      advance
      NextStmt.new
    when TokenKind::Exit
      advance
      expr = parse_expr rescue nil
      ExitStmt.new(expr)
    when TokenKind::Delete
      parse_delete
    else
      expr = parse_expr
      ExprStmt.new(expr)
    end
  end

  private def parse_expr : Expr
    parse_assign
  end

  private def parse_assign : Expr
    expr = parse_ternary
    if @curr.kind == TokenKind::Assign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignExpr.new(expr.name, rhs)
      elsif expr.is_a?(ArrayAccessExpr)
        return ArrayAssignExpr.new(expr.name, expr.index, rhs)
      elsif expr.is_a?(FieldExpr)
        return FieldAssignExpr.new(expr.index, rhs)
      elsif expr.is_a?(FieldDynExpr)
        return FieldDynAssignExpr.new(expr.inner, rhs)
      else
        raise "invalid assignment target"
      end
    elsif @curr.kind == TokenKind::PlusAssign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignOpExpr.new(expr.name, BinOpKind::Add, rhs)
      else
        raise "invalid assignment target"
      end
    elsif @curr.kind == TokenKind::MinusAssign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignOpExpr.new(expr.name, BinOpKind::Sub, rhs)
      else
        raise "invalid assignment target"
      end
    elsif @curr.kind == TokenKind::StarAssign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignOpExpr.new(expr.name, BinOpKind::Mul, rhs)
      else
        raise "invalid assignment target"
      end
    elsif @curr.kind == TokenKind::SlashAssign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignOpExpr.new(expr.name, BinOpKind::Div, rhs)
      else
        raise "invalid assignment target"
      end
    elsif @curr.kind == TokenKind::PercentAssign
      advance
      rhs = parse_assign
      if expr.is_a?(IdentExpr)
        return AssignOpExpr.new(expr.name, BinOpKind::Mod, rhs)
      else
        raise "invalid assignment target"
      end
    end
    expr
  end

  private def parse_ternary : Expr
    expr = parse_or
    if @curr.kind == TokenKind::Question
      advance
      true_branch = parse_expr
      expect(TokenKind::Colon)
      false_branch = parse_expr
      expr = TernaryExpr.new(expr, true_branch, false_branch)
    end
    expr
  end

  private def parse_or : Expr
    expr = parse_and
    while @curr.kind == TokenKind::Or
      advance
      rhs = parse_and
      expr = BinOpExpr.new(expr, BinOpKind::Or, rhs)
    end
    expr
  end

  private def parse_and : Expr
    expr = parse_compare
    while @curr.kind == TokenKind::And
      advance
      rhs = parse_compare
      expr = BinOpExpr.new(expr, BinOpKind::And, rhs)
    end
    expr
  end

  private def parse_compare : Expr
    expr = parse_match
    kind = case @curr.kind
           when TokenKind::Eq then BinOpKind::Eq
           when TokenKind::Ne then BinOpKind::Ne
           when TokenKind::Lt then BinOpKind::Lt
           when TokenKind::Gt then BinOpKind::Gt
           when TokenKind::Le then BinOpKind::Le
           when TokenKind::Ge then BinOpKind::Ge
           else                   nil
           end
    if kind
      advance
      rhs = parse_match
      expr = BinOpExpr.new(expr, kind, rhs)
    end
    expr
  end

  private def parse_match : Expr
    expr = parse_concat
    while @curr.kind == TokenKind::Tilde || @curr.kind == TokenKind::BangTilde
      kind = @curr.kind == TokenKind::Tilde ? BinOpKind::Match : BinOpKind::NotMatch
      advance
      saved = @auto_match_regex
      @auto_match_regex = false
      rhs = parse_concat
      @auto_match_regex = saved
      expr = BinOpExpr.new(expr, kind, rhs)
    end
    expr
  end

  private def parse_concat : Expr
    expr = parse_addsub
    while can_start_expr?
      rhs = parse_addsub
      expr = BinOpExpr.new(expr, BinOpKind::Concat, rhs)
    end
    expr
  end

  private def parse_addsub : Expr
    expr = parse_muldiv
    while @curr.kind == TokenKind::Plus || @curr.kind == TokenKind::Minus
      kind = @curr.kind == TokenKind::Plus ? BinOpKind::Add : BinOpKind::Sub
      advance
      rhs = parse_muldiv
      expr = BinOpExpr.new(expr, kind, rhs)
    end
    expr
  end

  private def parse_muldiv : Expr
    expr = parse_pow
    while @curr.kind == TokenKind::Star || @curr.kind == TokenKind::Slash || @curr.kind == TokenKind::Percent
      kind = case @curr.kind
             when TokenKind::Star   then BinOpKind::Mul
             when TokenKind::Slash  then BinOpKind::Div
             when TokenKind::Percent then BinOpKind::Mod
             else                        BinOpKind::Mul
             end
      advance
      rhs = parse_pow
      expr = BinOpExpr.new(expr, kind, rhs)
    end
    expr
  end

  private def parse_pow : Expr
    expr = parse_unary
    while @curr.kind == TokenKind::Caret
      advance
      rhs = parse_unary
      expr = BinOpExpr.new(expr, BinOpKind::Pow, rhs)
    end
    expr
  end

  private def parse_unary : Expr
    case @curr.kind
    when TokenKind::Not
      advance
      expr = parse_unary
      UnaryOpExpr.new(UnaryOpKind::Not, expr)
    when TokenKind::Minus
      advance
      expr = parse_unary
      UnaryOpExpr.new(UnaryOpKind::Neg, expr)
    when TokenKind::Plus
      advance
      parse_unary
    when TokenKind::Incr
      advance
      expr = parse_primary_with_incr
      if expr.is_a?(IdentExpr)
        IncrExpr.new(expr.name, 1, false)
      else
        raise "invalid increment target"
      end
    when TokenKind::Decr
      advance
      expr = parse_primary_with_incr
      if expr.is_a?(IdentExpr)
        IncrExpr.new(expr.name, -1, false)
      else
        raise "invalid decrement target"
      end
    else
      parse_primary_with_incr
    end
  end

  private def parse_primary_with_incr : Expr
    expr = parse_primary
    if @curr.kind == TokenKind::Incr
      advance
      if expr.is_a?(IdentExpr)
        IncrExpr.new(expr.name, 1, true)
      else
        raise "invalid increment target"
      end
    elsif @curr.kind == TokenKind::Decr
      advance
      if expr.is_a?(IdentExpr)
        IncrExpr.new(expr.name, -1, true)
      else
        raise "invalid decrement target"
      end
    else
      expr
    end
  end

  private def parse_primary : Expr
    case @curr.kind
    when TokenKind::Dollar
      advance
      inner = parse_primary
      FieldDynExpr.new(inner)
    when TokenKind::String
      val = @curr.str_value || ""
      advance
      StrExpr.new(val)
    when TokenKind::Regex
      val = @curr.str_value || ""
      advance
      if @auto_match_regex
        BinOpExpr.new(FieldExpr.new(0_u64), BinOpKind::Match, StrExpr.new(val))
      else
        StrExpr.new(val)
      end
    when TokenKind::Number
      val = @curr.num_value || 0.0
      advance
      NumExpr.new(val)
    when TokenKind::Ident
      name = @curr.str_value || ""
      advance
      if @curr.kind == TokenKind::LParen
        parse_call(name)
      elsif @curr.kind == TokenKind::LBracket
        advance
        index = parse_expr
        expect(TokenKind::RBracket)
        ArrayAccessExpr.new(name, index)
      else
        IdentExpr.new(name)
      end
    when TokenKind::LParen
      advance
      expr = parse_expr
      expect(TokenKind::RParen)
      expr
    when TokenKind::Invalid
      ch = @curr.str_value || ""
      advance
      raise "invalid character: #{ch}"
    else
      raise err_unexpected
    end
  end

  private def parse_call(name : String) : Expr
    advance
    args = [] of Expr
    saved_auto_match = @auto_match_regex
    @auto_match_regex = false
    if @curr.kind != TokenKind::RParen
      args << parse_expr
      while @curr.kind == TokenKind::Comma
        advance
        args << parse_expr
      end
    end
    expect(TokenKind::RParen)
    @auto_match_regex = saved_auto_match
    CallExpr.new(name, args)
  end
end

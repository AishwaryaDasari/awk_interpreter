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
         TokenKind::Not
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
    stmts = [] of Stmt
    loop do
      case @curr.kind
      when TokenKind::RBrace, TokenKind::EOF
        break
      when TokenKind::Semicolon
        advance
      when TokenKind::Print
        stmts << parse_print
      else
        raise err_unexpected
      end
    end
    expect(TokenKind::RBrace)
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

  private def parse_expr : Expr
    parse_ternary
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
    expr = parse_unary
    while can_start_expr?
      rhs = parse_unary
      expr = BinOpExpr.new(expr, BinOpKind::Concat, rhs)
    end
    expr
  end

  private def parse_unary : Expr
    if @curr.kind == TokenKind::Not
      advance
      expr = parse_unary
      UnaryOpExpr.new(UnaryOpKind::Not, expr)
    else
      parse_primary
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
      IdentExpr.new(name)
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
end

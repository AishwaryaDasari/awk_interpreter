enum TokenKind
  Print
  Begin
  End
  BeginFile
  EndFile
  LBrace
  RBrace
  LParen
  RParen
  Dollar
  Comma
  Semicolon
  Tilde
  BangTilde
  Eq
  Ne
  Lt
  Gt
  Le
  Ge
  And
  Or
  Not
  Question
  Colon
  Regex
  String
  Number
  Ident
  Invalid
  EOF
end

class Token
  getter kind : TokenKind
  getter str_value : String?
  getter num_value : Float64?

  def initialize(@kind, @str_value = nil, @num_value = nil)
  end

  def ==(other : Token) : Bool
    @kind == other.kind &&
      @str_value == other.str_value &&
      @num_value == other.num_value
  end
end

class Lexer
  getter input : String
  getter pos : Int32

  def initialize(@input)
    @pos = 0
  end

  def next_token : Token
    skip_whitespace
    return Token.new(TokenKind::EOF) if @pos >= @input.size

    ch = @input[@pos]
    case ch
    when '{'
      @pos += 1
      Token.new(TokenKind::LBrace)
    when '}'
      @pos += 1
      Token.new(TokenKind::RBrace)
    when '('
      @pos += 1
      Token.new(TokenKind::LParen)
    when ')'
      @pos += 1
      Token.new(TokenKind::RParen)
    when '$'
      @pos += 1
      Token.new(TokenKind::Dollar)
    when ','
      @pos += 1
      Token.new(TokenKind::Comma)
    when ';'
      @pos += 1
      Token.new(TokenKind::Semicolon)
    when '?'
      @pos += 1
      Token.new(TokenKind::Question)
    when ':'
      @pos += 1
      Token.new(TokenKind::Colon)
    when '~'
      @pos += 1
      Token.new(TokenKind::Tilde)
    when '!'
      @pos += 1
      if @pos < @input.size
        case @input[@pos]
        when '='
          @pos += 1
          Token.new(TokenKind::Ne)
        when '~'
          @pos += 1
          Token.new(TokenKind::BangTilde)
        else
          Token.new(TokenKind::Not)
        end
      else
        Token.new(TokenKind::Not)
      end
    when '='
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::Eq)
      else
        Token.new(TokenKind::Invalid, "=")
      end
    when '>'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::Ge)
      else
        Token.new(TokenKind::Gt)
      end
    when '<'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::Le)
      else
        Token.new(TokenKind::Lt)
      end
    when '&'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '&'
        @pos += 1
        Token.new(TokenKind::And)
      else
        Token.new(TokenKind::Invalid, "'&'")
      end
    when '|'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '|'
        @pos += 1
        Token.new(TokenKind::Or)
      else
        Token.new(TokenKind::Invalid, "'|'")
      end
    when '/'
      read_regex
    when '"'
      read_string
    when '0'..'9', '-'
      read_number
    when 'a'..'z', 'A'..'Z', '_'
      read_ident
    else
      @pos += 1
      Token.new(TokenKind::Invalid, ch.to_s)
    end
  end

  private def skip_whitespace
    while @pos < @input.size && @input[@pos].ascii_whitespace?
      @pos += 1
    end
  end

  private def read_regex : Token
    @pos += 1
    start = @pos
    while @pos < @input.size && @input[@pos] != '/'
      @pos += 1
    end
    s = @input[start...@pos]
    @pos += 1 if @pos < @input.size
    Token.new(TokenKind::Regex, s)
  end

  private def read_string : Token
    @pos += 1
    start = @pos
    while @pos < @input.size && @input[@pos] != '"'
      @pos += 1
    end
    s = @input[start...@pos]
    @pos += 1 if @pos < @input.size
    Token.new(TokenKind::String, s)
  end

  private def read_number : Token
    start = @pos
    if @input[@pos] == '-'
      @pos += 1
    end
    while @pos < @input.size && @input[@pos] >= '0' && @input[@pos] <= '9'
      @pos += 1
    end
    if @pos < @input.size && @input[@pos] == '.'
      @pos += 1
      while @pos < @input.size && @input[@pos] >= '0' && @input[@pos] <= '9'
        @pos += 1
      end
    end
    s = @input[start...@pos]
    n = s.to_f? || 0.0
    Token.new(TokenKind::Number, num_value: n)
  end

  private def read_ident : Token
    start = @pos
    while @pos < @input.size && (@input[@pos].alphanumeric? || @input[@pos] == '_')
      @pos += 1
    end
    s = @input[start...@pos]
    kind = case s
           when "print"    then TokenKind::Print
           when "BEGIN"    then TokenKind::Begin
           when "END"      then TokenKind::End
           when "BEGINFILE" then TokenKind::BeginFile
           when "ENDFILE"  then TokenKind::EndFile
           else                 TokenKind::Ident
           end
    if kind == TokenKind::Ident
      Token.new(kind, s)
    else
      Token.new(kind)
    end
  end
end

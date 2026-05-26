enum TokenKind
  Print
  Printf
  Begin
  End
  BeginFile
  EndFile
  If
  Else
  While
  For
  In
  Break
  Continue
  Next
  Exit
  Delete
  LBrace
  RBrace
  LParen
  RParen
  LBracket
  RBracket
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
  Plus
  Minus
  Star
  Slash
  Percent
  Caret
  Assign
  PlusAssign
  MinusAssign
  StarAssign
  SlashAssign
  PercentAssign
  Incr
  Decr
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

module TokenHelper
  KEYWORDS = {
    "print"     => TokenKind::Print,
    "printf"    => TokenKind::Printf,
    "BEGIN"     => TokenKind::Begin,
    "END"       => TokenKind::End,
    "BEGINFILE" => TokenKind::BeginFile,
    "ENDFILE"   => TokenKind::EndFile,
    "if"        => TokenKind::If,
    "else"      => TokenKind::Else,
    "while"     => TokenKind::While,
    "for"       => TokenKind::For,
    "in"        => TokenKind::In,
    "break"     => TokenKind::Break,
    "continue"  => TokenKind::Continue,
    "next"      => TokenKind::Next,
    "exit"      => TokenKind::Exit,
    "delete"    => TokenKind::Delete,
  }
end

class Lexer
  getter input : String
  property pos : Int32

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
    when '['
      @pos += 1
      Token.new(TokenKind::LBracket)
    when ']'
      @pos += 1
      Token.new(TokenKind::RBracket)
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
    when '^'
      @pos += 1
      Token.new(TokenKind::Caret)
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
        Token.new(TokenKind::Assign)
      end
    when '>'
      @pos += 1
      if @pos < @input.size
        case @input[@pos]
        when '='
          @pos += 1
          Token.new(TokenKind::Ge)
        when '>'
          @pos += 1
          Token.new(TokenKind::Gt)
        else
          Token.new(TokenKind::Gt)
        end
      else
        Token.new(TokenKind::Gt)
      end
    when '<'
      @pos += 1
      if @pos < @input.size
        case @input[@pos]
        when '='
          @pos += 1
          Token.new(TokenKind::Le)
        else
          Token.new(TokenKind::Lt)
        end
      else
        Token.new(TokenKind::Lt)
      end
    when '&'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '&'
        @pos += 1
        Token.new(TokenKind::And)
      else
        Token.new(TokenKind::Invalid, "&")
      end
    when '|'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '|'
        @pos += 1
        Token.new(TokenKind::Or)
      else
        Token.new(TokenKind::Invalid, "|")
      end
    when '+'
      @pos += 1
      if @pos < @input.size
        case @input[@pos]
        when '+'
          @pos += 1
          Token.new(TokenKind::Incr)
        when '='
          @pos += 1
          Token.new(TokenKind::PlusAssign)
        else
          Token.new(TokenKind::Plus)
        end
      else
        Token.new(TokenKind::Plus)
      end
    when '-'
      @pos += 1
      if @pos < @input.size
        case @input[@pos]
        when '-'
          @pos += 1
          Token.new(TokenKind::Decr)
        when '='
          @pos += 1
          Token.new(TokenKind::MinusAssign)
        else
          Token.new(TokenKind::Minus)
        end
      else
        Token.new(TokenKind::Minus)
      end
    when '*'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::StarAssign)
      else
        Token.new(TokenKind::Star)
      end
    when '/'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::SlashAssign)
      else
        @pos -= 1
        read_regex
      end
    when '%'
      @pos += 1
      if @pos < @input.size && @input[@pos] == '='
        @pos += 1
        Token.new(TokenKind::PercentAssign)
      else
        Token.new(TokenKind::Percent)
      end
    when '"'
      read_string
    when '0'..'9'
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
    result = String::Builder.new
    while @pos < @input.size
      ch = @input[@pos]
      if ch == '"'
        break
      elsif ch == '\\'
        @pos += 1
        if @pos < @input.size
          case @input[@pos]
          when 'n'  then result << '\n'
          when 't'  then result << '\t'
          when 'r'  then result << '\r'
          when '\\' then result << '\\'
          when '"'  then result << '"'
          when 'a'  then result << '\a'
          when 'b'  then result << '\b'
          when 'f'  then result << '\f'
          when 'v'  then result << '\v'
          when '0'..'9'
            octal = @input[@pos] - '0'
            @pos += 1
            count = 1
            while count < 3 && @pos < @input.size && @input[@pos] >= '0' && @input[@pos] <= '7'
              octal = octal * 8 + (@input[@pos] - '0')
              count += 1
              @pos += 1 if count < 3
            end
            result << octal.chr
            @pos -= 1 if count >= 3
          when 'x'
            @pos += 1
            hex = 0
            count = 0
            while count < 2 && @pos < @input.size
              c = @input[@pos]
              if c >= '0' && c <= '9'
                hex = hex * 16 + (c - '0')
              elsif c >= 'a' && c <= 'f'
                hex = hex * 16 + (c - 'a') + 10
              elsif c >= 'A' && c <= 'F'
                hex = hex * 16 + (c - 'A') + 10
              else
                break
              end
              count += 1
              @pos += 1
            end
            result << hex.chr
          else
            result << '\\' << @input[@pos]
          end
        end
      else
        result << ch
      end
      @pos += 1
    end
    @pos += 1 if @pos < @input.size && @input[@pos] == '"'
    Token.new(TokenKind::String, result.to_s)
  end

  private def read_number : Token
    start = @pos
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
    kind = TokenHelper::KEYWORDS[s]? || TokenKind::Ident
    if kind == TokenKind::Ident
      Token.new(kind, s)
    else
      Token.new(kind)
    end
  end
end

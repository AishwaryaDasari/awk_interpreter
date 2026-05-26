abstract class Node
end

class Program < Node
  getter rules : Array(Rule)

  def initialize(@rules)
  end
end

abstract class Rule
end

class Begin < Rule
  getter stmts : Array(Stmt)

  def initialize(@stmts)
  end
end

class End < Rule
  getter stmts : Array(Stmt)

  def initialize(@stmts)
  end
end

class BeginFile < Rule
  getter stmts : Array(Stmt)

  def initialize(@stmts)
  end
end

class EndFile < Rule
  getter stmts : Array(Stmt)

  def initialize(@stmts)
  end
end

class PatternRule < Rule
  getter pattern : Pattern
  getter stmts : Array(Stmt)

  def initialize(@pattern, @stmts)
  end
end

abstract class Pattern
end

class ExprPattern < Pattern
  getter expr : Expr

  def initialize(@expr)
  end
end

class RangePattern < Pattern
  getter start : Expr
  getter end_expr : Expr

  def initialize(@start, @end_expr)
  end
end

abstract class Stmt
end

class PrintStmt < Stmt
  getter exprs : Array(Expr)

  def initialize(@exprs)
  end
end

enum BinOpKind
  Eq
  Ne
  Lt
  Gt
  Le
  Ge
  And
  Or
  Match
  NotMatch
  Concat

end

enum UnaryOpKind
  Not
end

abstract class Expr
end

class FieldExpr < Expr
  getter index : UInt64

  def initialize(@index)
  end
end

class FieldDynExpr < Expr
  getter inner : Expr

  def initialize(@inner)
  end
end

class IdentExpr < Expr
  getter name : String

  def initialize(@name)
  end
end

class StrExpr < Expr
  getter value : String

  def initialize(@value)
  end
end

class NumExpr < Expr
  getter value : Float64

  def initialize(@value)
  end
end

class BinOpExpr < Expr
  getter lhs : Expr
  getter kind : BinOpKind
  getter rhs : Expr

  def initialize(@lhs, @kind, @rhs)
  end
end

class UnaryOpExpr < Expr
  getter kind : UnaryOpKind
  getter expr : Expr

  def initialize(@kind, @expr)
  end
end

class TernaryExpr < Expr
  getter cond : Expr
  getter true_branch : Expr
  getter false_branch : Expr

  def initialize(@cond, @true_branch, @false_branch)
  end
end

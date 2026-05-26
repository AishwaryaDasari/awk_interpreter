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

class PrintfStmt < Stmt
  getter fmt : Expr
  getter args : Array(Expr)

  def initialize(@fmt, @args)
  end
end

class ExprStmt < Stmt
  getter expr : Expr

  def initialize(@expr)
  end
end

class IfStmt < Stmt
  getter cond : Expr
  getter then_branch : Array(Stmt)
  getter else_branch : Array(Stmt)?

  def initialize(@cond, @then_branch, @else_branch = nil)
  end
end

class WhileStmt < Stmt
  getter cond : Expr
  getter body : Array(Stmt)

  def initialize(@cond, @body)
  end
end

class ForStmt < Stmt
  getter init : Stmt?
  getter cond : Expr?
  getter update : Expr?
  getter body : Array(Stmt)

  def initialize(@init, @cond, @update, @body)
  end
end

class ForInStmt < Stmt
  getter var : String
  getter array_name : String
  getter body : Array(Stmt)

  def initialize(@var, @array_name, @body)
  end
end

class BreakStmt < Stmt
end

class ContinueStmt < Stmt
end

class NextStmt < Stmt
end

class ExitStmt < Stmt
  getter expr : Expr?

  def initialize(@expr = nil)
  end
end

class DeleteStmt < Stmt
  getter name : String
  getter index : Expr?

  def initialize(@name, @index = nil)
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
  Add
  Sub
  Mul
  Div
  Mod
  Pow
end

enum UnaryOpKind
  Not
  Neg
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

class AssignExpr < Expr
  getter name : String
  getter value : Expr

  def initialize(@name, @value)
  end
end

class AssignOpExpr < Expr
  getter name : String
  getter op : BinOpKind
  getter value : Expr

  def initialize(@name, @op, @value)
  end
end

class IncrExpr < Expr
  getter name : String
  getter delta : Int32
  getter is_post : Bool

  def initialize(@name, @delta, @is_post)
  end
end

class CallExpr < Expr
  getter name : String
  getter args : Array(Expr)

  def initialize(@name, @args)
  end
end

class ArrayAccessExpr < Expr
  getter name : String
  getter index : Expr

  def initialize(@name, @index)
  end
end

class ArrayAssignExpr < Expr
  getter name : String
  getter index : Expr
  getter value : Expr

  def initialize(@name, @index, @value)
  end
end

class FieldAssignExpr < Expr
  getter index : UInt64
  getter value : Expr

  def initialize(@index, @value)
  end
end

class FieldDynAssignExpr < Expr
  getter inner : Expr
  getter value : Expr

  def initialize(@inner, @value)
  end
end

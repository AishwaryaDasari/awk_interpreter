class AwkValue
  getter raw : String | Float64

  def initialize(@raw : String | Float64)
  end

  def to_s : String
    case v = @raw
    when String
      v
    when Float64
      if v == v.floor
        v.to_i.to_s
      else
        v.to_s
      end
    else
      ""
    end
  end

  def as_f64 : Float64
    case v = @raw
    when Float64
      v
    when String
      v.to_f? || 0.0
    else
      0.0
    end
  end

  def truthy? : Bool
    case v = @raw
    when Float64
      v != 0.0
    when String
      !v.empty?
    else
      false
    end
  end

  def ==(other : AwkValue) : Bool
    a = @raw
    b = other.raw
    case {a, b}
    when {Float64, Float64}
      (a - b).abs < Float64::EPSILON
    when {String, String}
      a == b
    when {Float64, String}
      a.to_s == b
    when {String, Float64}
      a == b.to_s
    else
      false
    end
  end
end

class Env
  getter fields : Array(AwkValue)
  getter nr : Int32
  getter filename : String
  getter vars : Hash(String, AwkValue)

  def initialize
    @fields = [] of AwkValue
    @nr = 0
    @filename = ""
    @vars = Hash(String, AwkValue).new
  end

  def set_filename(name : String)
    @filename = name
  end

  def set_line(line : String, field_sep : String)
    @nr += 1
    raw = if field_sep == " "
            line.split(/\s+/)
          else
            line.split(field_sep)
          end
    fields = [] of AwkValue
    fields << AwkValue.new(line)
    raw.each { |f| fields << AwkValue.new(f) }
    @fields = fields
  end

  def get_field(n : Int32) : AwkValue
    if n < @fields.size
      @fields[n]
    else
      AwkValue.new("")
    end
  end

  def get_field(n : UInt64) : AwkValue
    get_field(n.to_i32)
  end

  def get_var(name : String) : AwkValue
    case name
    when "NF"
      nf = {@fields.size - 1, 0}.max
      AwkValue.new(nf.to_f64)
    when "NR"
      AwkValue.new(@nr.to_f64)
    when "FILENAME"
      AwkValue.new(@filename)
    else
      @vars.fetch(name) { AwkValue.new(0.0) }
    end
  end
end

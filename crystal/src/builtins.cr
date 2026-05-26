class Env
  getter fields : Array(AwkValue)
  getter nr : Int32
  getter fnr : Int32
  getter filename : String
  getter vars : Hash(String, AwkValue)
  getter arrays : Hash(String, Hash(String, AwkValue))
  property fs : String
  property ofs : String
  property ors : String

  def initialize
    @fields = [] of AwkValue
    @nr = 0
    @fnr = 0
    @filename = ""
    @vars = Hash(String, AwkValue).new
    @arrays = Hash(String, Hash(String, AwkValue)).new
    @fs = " "
    @ofs = " "
    @ors = "\n"
  end

  def set_filename(name : String)
    @filename = name
    @fnr = 0
  end

  def set_line(line : String, field_sep : String)
    @nr += 1
    @fnr += 1
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
    if n >= 0 && n < @fields.size
      @fields[n]
    else
      AwkValue.new("")
    end
  end

  def get_field(n : UInt64) : AwkValue
    get_field(n.to_i32)
  end

  def set_field(n : Int32, value : AwkValue)
    if n == 0
      line = value.to_s
      raw = if @fs == " "
              line.split(/\s+/)
            else
              line.split(@fs)
            end
      new_fields = [] of AwkValue
      new_fields << AwkValue.new(line)
      raw.each { |f| new_fields << AwkValue.new(f) }
      @fields = new_fields
    elsif n >= 1 && n < @fields.size
      @fields[n] = value
      @fields[0] = AwkValue.new(@fields[1..].map(&.to_s).join(@ofs))
    end
  end

  def get_var(name : String) : AwkValue
    case name
    when "NF"
      nf = {@fields.size - 1, 0}.max
      AwkValue.new(nf.to_f64)
    when "NR"
      AwkValue.new(@nr.to_f64)
    when "FNR"
      AwkValue.new(@fnr.to_f64)
    when "FILENAME"
      AwkValue.new(@filename)
    when "FS"
      AwkValue.new(@fs)
    when "OFS"
      AwkValue.new(@ofs)
    when "ORS"
      AwkValue.new(@ors)
    when "RS"
      AwkValue.new("\n")
    else
      @vars.fetch(name) { AwkValue.new(0.0) }
    end
  end

  def set_var(name : String, value : AwkValue)
    case name
    when "FS"
      @fs = value.to_s
    when "OFS"
      @ofs = value.to_s
    when "ORS"
      @ors = value.to_s
    else
      @vars[name] = value
    end
  end

  def array_get(name : String, index : AwkValue) : AwkValue
    key = index.to_s
    arr = @arrays[name]?
    if arr
      arr.fetch(key) { AwkValue.new("") }
    else
      AwkValue.new("")
    end
  end

  def array_set(name : String, index : AwkValue, value : AwkValue)
    key = index.to_s
    @arrays[name] ||= Hash(String, AwkValue).new
    @arrays[name][key] = value
  end

  def array_delete(name : String, index : AwkValue?)
    if i = index
      key = i.to_s
      if arr = @arrays[name]?
        arr.delete(key)
      end
    else
      @arrays.delete(name)
    end
  end

  def array_keys(name : String) : Array(String)
    arr = @arrays[name]?
    arr ? arr.keys : [] of String
  end
end

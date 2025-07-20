# Ruby 3.0+ compatibility patch for Jekyll and Liquid 4.0.3
# The tainted?/untaint methods were removed in Ruby 3.0 but Liquid 4.0.3 still uses them
# This patch adds dummy implementations that make it work with Ruby 3.0+

unless String.method_defined?(:tainted?)
  class String
    def tainted?
      false
    end
    
    def untaint
      self
    end
  end
end

unless Object.method_defined?(:taint)
  class Object
    def taint
      self
    end
    
    def untaint
      self
    end
    
    def tainted?
      false
    end
  end
end

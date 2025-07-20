# Ruby 3.0 compatibility patch for Liquid 4.0.3
# The tainted? method was removed in Ruby 3.0 but Liquid 4.0.3 still uses it
# This patch adds a dummy implementation that always returns false

unless String.method_defined?(:tainted?)
  class String
    def tainted?
      false
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

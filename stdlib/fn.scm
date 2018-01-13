(defn curry (f arg)
      (fn (. args)
          (apply f (cons arg args))))

(defn words (s) (string-split " " s))
(defn lines (s) (string-split "\n" s))

(defn empty? (lst)
      (zero? (length lst)))

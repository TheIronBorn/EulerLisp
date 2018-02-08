; Solved 1.2.2018
; Time: 3:44

(defn num-factors (n)
  (~> n factors length))

(~>
  (range~ 2 9_999_999)
  (map~ num-factors)
  (reduce~
    (fn (cur acc)
        (cons cur
              (if (= (fst acc) cur) (inc (rst acc))
                (rst acc))))
    (cons 0 0))
  rst
  solution)

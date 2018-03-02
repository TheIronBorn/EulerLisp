; Solved: 17.12.2017

(defn solve (a b)
  (defn inner (a b n)
    (if (>= (number-of-digits a) 1000)
      n
      (inner b (+ a b) (inc n))))
  (inner a b 1))


(solution (solve 1 1))

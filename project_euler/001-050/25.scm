; Solved: 17.12.2017

(defn solve ((a 1) (b 1) (n 1))
      (if (>= (number-of-digits a) 1000)
          n
          (solve b (+ a b) (inc n))))

(solution (solve))

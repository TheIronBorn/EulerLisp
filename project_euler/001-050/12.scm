; Solved: 29.12.2017

(defn solve ((number 1) (n 2))
      (if (~> number factors length (< 500))
        number
        (solve (+ number n) (inc n))))

(solution (solve))

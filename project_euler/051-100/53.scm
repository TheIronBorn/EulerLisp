; Solved 14.1.2018

(defn solve (r n greater)
  (cond
    [(> r 100) greater]
    [(> n 100) (solve (inc r) (inc r) greater)]
    [else
     (if (> (binomial n r) 1000000)
         (solve (inc r) (inc r)
                (+ greater (- 100 (- n 1))))
         (solve r (inc n) greater))]))

(solution (solve 1 1 0))

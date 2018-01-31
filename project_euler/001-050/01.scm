; Solved: 17.12.17

(def n 1000)

(solution
  (+ (* 3 (gauss-sum (div n 3)))
     (* 5 (gauss-sum (div n 5)))
     (- (* 15 (gauss-sum (div n 15))))))

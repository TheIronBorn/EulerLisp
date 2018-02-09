; Solved 8.1.2018

(defn solve ((cur 1) (streak 0))
  (cond
    [(= streak 4) (- cur 4)]
    [(>= (length (prime-factors cur)) 4)
      (solve (inc cur) (inc streak))]
    [else (solve (inc cur) 0)]))

(solution (solve))

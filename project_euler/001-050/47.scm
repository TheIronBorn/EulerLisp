; Solved 8.1.2018

(defn solve ()
  (defn inner (cur streak)
    (cond
      [(= streak 4) (- cur 4)]
      [(>= (length (prime-factors cur)) 4)
       (inner (inc cur) (inc streak))]
      [else (inner (inc cur) 0)]))
  (inner 1 0))

(solution (solve))

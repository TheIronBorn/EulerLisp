; Solved 8.1

(defn goldbach? (n (sq 1))
  (cond
    (< n (* 2 sq sq)) #f
    (prime? (- n (* 2 sq sq))) #t
    else (goldbach? n (inc sq))))

(defn solve (cur)
  (if (or (prime? cur) (goldbach? cur))
      (solve (+ cur 2))
      cur))

(println "Solution: " (solve 9))

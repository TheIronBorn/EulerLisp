; Solved 6.1

(defn rotations (n)
  (cond
    (< n 10)
    (list)
    (< n 100)
    (list
      (+ (* (% n 10) 10) (/ n 10)))
    (< n 1000)
    (list
      (+ (* (% n 100) 10) (/ n 100))
      (+ (* (% n 10) 100) (/ n 10)))
    (< n 10000)
    (list
      (+ (* (% n 1000) 10) (/ n 1000))
      (+ (* (% n 100) 100) (/ n 100))
      (+ (* (% n 10) 1000) (/ n 10)))
    (< n 100000)
    (list
      (+ (* (% n 10000) 10) (/ n 10000))
      (+ (* (% n 1000) 100) (/ n 1000))
      (+ (* (% n 100) 1000) (/ n 100))
      (+ (* (% n 10) 10000) (/ n 10)))
    (< n 1000000)
    (list
      (+ (* (% n 100000) 10) (/ n 100000))
      (+ (* (% n 10000) 100) (/ n 10000))
      (+ (* (% n 1000) 1000) (/ n 1000))
      (+ (* (% n 100) 10000) (/ n 100))
      (+ (* (% n 10) 100000) (/ n 10)))
    else (println "Error, number is to big")))

(defn rotatable-prime? (n)
  (and (prime? n)
       (all? prime? (rotations n))))

; This is based on the assumption that all 11 trunc primes are < 1000000
(defn solve (cur (acc 1))
  (if (>= cur 1000000)
      acc
      (if (rotatable-prime? cur)
          (do
            (println "Rotatable prime: " cur)
            (solve (+ cur 2) (inc acc)))
          (solve (+ cur 2) acc))))

(println "Solution: " (solve 3))

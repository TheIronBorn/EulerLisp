; Solved 6.1.2018

(defn rotations (n)
      (cond
        [(< n 10) (list)]
        [(< n 100)
         (list (+ (* (% n 10) 10) (div n 10)))]
        [(< n 1000)
         (list
           (+ (* (% n 100) 10) (div n 100))
           (+ (* (% n 10) 100) (div n 10)))]
        [(< n 10000)
         (list
           (+ (* (% n 1000) 10) (div n 1000))
           (+ (* (% n 100) 100) (div n 100))
           (+ (* (% n 10) 1000) (div n 10)))]
        [(< n 100000)
         (list
           (+ (* (% n 10000) 10) (div n 10000))
           (+ (* (% n 1000) 100) (div n 1000))
           (+ (* (% n 100) 1000) (div n 100))
           (+ (* (% n 10) 10000) (div n 10)))]
        [(< n 1_000_000)
         (list
           (+ (* (% n 100000) 10) (div n 100000))
           (+ (* (% n 10000) 100) (div n 10000))
           (+ (* (% n 1000) 1000) (div n 1000))
           (+ (* (% n 100) 10000) (div n 100))
           (+ (* (% n 10) 100000) (div n 10)))]
        [else (println "Error, number is to big")]))

(defn rotatable-prime? (n)
  (and (prime? n)
       (all? prime? (rotations n))))

(~>
  (range~ 3 999_999 2)
  (count~ rotatable-prime?)
  inc ; 2 is a rotatable prime, too
  solution)

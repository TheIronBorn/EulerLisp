; Solved 8.1.2018

(defn goldbach? (n)
  (defn inner (sq)
    (cond
      [(< n (* 2 sq sq)) #f]
      [(prime? (- n (* 2 sq sq))) #t]
      [else (inner (inc sq))]))
  (inner 1))

(~> (step-stream 9 2)
    (stream-select &(not (or (prime? &1) (goldbach? &1))))
    fst
    solution)

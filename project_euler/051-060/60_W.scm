(defn concat (a b)
  (+ b
     (* (pow 10 (floor (log10 b)))
        10
        a)))

(defn concat-primes? (a b)
      (and
        (prime? (concat a b))
        (prime? (concat b a))))

(println (concat-primes? 3 7))

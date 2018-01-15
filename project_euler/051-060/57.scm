; Solved 14.1

(defn next (frac)
  (cons
    (bignum+ (fst frac)
       (bignum* (bignum 2) (rst frac)))
    (bignum+ (fst frac)
       (rst frac))))

(defn more-digits? (frac)
  (>
    (~> frac fst bignum-num-digits)
    (~> frac rst bignum-num-digits)))

(defn solve (cur (n 1000) (acc 0))
  (println cur)
  (if (zero? n)
      acc
      (solve (next cur) (dec n)
             (if (more-digits? cur) (inc acc) acc))))

(println (solve (cons (bignum 3) (bignum 2))))

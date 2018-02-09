; Solved 14.1.2018

(defn next (frac)
  (cons
    (+ (fst frac) (* 2 (rst frac)))
    (+ (fst frac) (rst frac))))

(defn more-digits? (frac)
  (> (~> frac fst number-of-digits)
     (~> frac rst number-of-digits)))

(defn solve (cur (n 1000) (acc 0))
  (if (zero? n)
      acc
      (solve (next cur) (dec n)
             (if (more-digits? cur) (inc acc) acc))))

(~> (cons 3 2) solve solution)

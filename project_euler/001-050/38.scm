; Solved 8.1.2018

(def all-digits (range 1 9))
(defn pandigital? (lst)
  (= (sort lst) all-digits))

(defn solve (n (x 1) (max-pan 0))
  (let ([ds (flatmap 
              &(reverse (number->digits (* x &1)))
              (range 1 n))])
    (if (> (length ds) 9)
      (if (= n 9)
          max-pan
          (solve (inc n) 1 max-pan))
      (if (pandigital? ds)
          (solve n (inc x)
                 (max max-pan
                      (digits->number (reverse ds))))
          (solve n (inc x) max-pan)))))

(solution (solve 2))

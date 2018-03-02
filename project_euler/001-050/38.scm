; Solved 8.1.2018

(def all-digits (range 1 9))
(defn pandigital? (lst)
  (= (sort lst) all-digits))

(defn solve (n)
  (defn inner (n x max-pan)
    (let ([ds (flatmap 
                &(reverse (number->digits (* x &1)))
                (range 1 n))])
      (if (> (length ds) 9)
        (if (= n 9)
            max-pan
            (inner (inc n) 1 max-pan))
        (if (pandigital? ds)
            (inner n (inc x)
                   (max max-pan
                        (digits->number (reverse ds))))
            (inner n (inc x) max-pan)))))
  (inner n 1 0))

(solution (solve 2))

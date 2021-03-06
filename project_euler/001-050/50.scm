; Solved 13.1.2018

(defn solve-from (n acc best terms best-terms)
  (if (prime? n)
      (let ([next (+ n acc)])
        (if (>= next 1000000)
            (cons best best-terms)
            (if (prime? next)
              (solve-from
                (+ n 1)
                next
                next
                (inc terms)
                (inc terms))
              (solve-from
                (+ n 1)
                next
                best
                (inc terms)
                best-terms)
        )))
      (solve-from (+ n 1) acc best terms best-terms)))

(defn solve ()
  (defn inner (from best best-terms)
    (if (>= from 1000000) 
        (cons best best-terms)
        (if (prime? from)
            (let ([res (solve-from from 0 0 0 0)])
              (if (> (rst res) best-terms)
                  (inner (inc from) (fst res) (rst res))
                  (inner (inc from) best best-terms)))
            (inner (inc from) best best-terms))))
  (inner 0 0 0))

(solution (solve))

; Solved 25.1.18,
; improved to run in < 60s on 31.1.18

; phi(n) = number of divisors of n with gcd(d, n) = 1

(def n 1000000)

(def tots (filled-list (inc n) 0))
(defn fill (cur)
  (when (<= cur n)
    (set-nth! tots cur cur)
    (fill (inc cur))))
(defn update (p k n)
  (when (<= k n)
    (let ([old (list-ref tots k)])
      (set-nth! tots k (- old (div old p))))
    (update p (+ k p) n)))
(defn loop (p acc)
  (if (<= p n)
      (do
        (if (= p (list-ref tots p)) (update p p n))
          (loop
            (inc p)
            (+ acc (- p (list-ref tots p)))))
    acc))
(fill 0)
(loop 2 0)

(~>
  (range~ 2 1000000)
  (map~ &(list-ref tots &1))
  sum~
  (println "Solution: "))

; Solved: 28.1.2018

(def ps #())

(~> (range~ 3 50_000_000 2)
    (select~ prime?)
    (reduce~ (fn (p acc) (vector-push! ps p)) '()))

; No need to check if i < (vector-length ps),
; there are primes > 10e8 / 3
; so for all primes the product will be > 10e8
; before reaching the end of the array
(defn loop (p1 i (acc 0))
  (let ([prod (* p1 (vector-ref ps i))])
    (if (> prod 100000000)
      acc
      (loop p1 (inc i) (inc acc)))))

; Add all products 2 * other prime
; and 2 * 2
(~> (range~ 0 (dec (vector-length ps)))
    (map~ &(loop (vector-ref ps &1) &1))
    sum~
    (+ (vector-length ps) 1)
    solution)

; Solved 27.1.2018

; Assume that P(0) = 1
(def partitions #(1))
(defn lookup (n) (if (>= n 0) (vector-ref partitions n) 0))

; This uses the recursive formula w/ pantagonal numbers
; from wikipedia
(defn next-partitions (n (i 1) (acc 0))
  (let ([p1 (pentagonal i)])
    (if (> p1 n)
        acc
        (next-partitions n (inc i)
              (if (even? i)
                      (+ acc
                         (- (lookup (- n p1)))
                         (- (lookup (- n (+ p1 i)))))
                      (+ acc
                         (lookup (- n p1))
                         (lookup (- n (+ p1 i)))))))))

; Because we are working % 1000000 everywhere,
; it is possible that P(n) becomes negative,
; due to the subtractions.
; This doesn't affect the result but we can clean it up anyway
(defn loop (from)
  (let ([p (% (next-partitions from) 1000000)])
    (vector-push! partitions p)
    (if (zero? p)
        from
        (loop (inc from)))))

(solution (loop 1))

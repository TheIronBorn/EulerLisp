; Solved 31.12.2017

(def known #(0 1 2))

(defn collatz-length (n)
  (defn inner (n len)
    (if (< n (vector-length known))
        (+ len (vector-ref known n))
        (case (% n 4)
          [0 (inner (div n 4) (+ len 2))]
          [1 (inner (+ (* 3 (div n 4)) 1) (+ len 3))]
          [2 (inner (+ (* 3 (div n 4)) 2) (+ len 3))]
          [3 (inner (+ (* 9 (div n 4)) 8) (+ len 4))])))
  (inner n 1))

; We can be pretty sure that the number with the max length will be odd
(defn solve (from)
  (defn inner (from max-n max-len)
      (if (> from 1000000)
          max-n
          (let ([cur-len (collatz-length from)])
            (vector-push! known cur-len)
            (if (> cur-len max-len)
              (inner (inc from) from cur-len)
              (inner (inc from) max-n max-len)))))
  (inner from 0 0))

(solution (solve 3))

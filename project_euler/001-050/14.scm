; Solved 31.12.2017

(def known #(0 1 2))

(defn collatz-length (n (len 1))
  (if (< n (vector-length known))
      (+ len (vector-ref known n))
      (case (% n 4)
        [0 (collatz-length (div n 4) (+ len 2))]
        [1 (collatz-length (+ (* 3 (div n 4)) 1) (+ len 3))]
        [2 (collatz-length (+ (* 3 (div n 4)) 2) (+ len 3))]
        [3 (collatz-length (+ (* 9 (div n 4)) 8) (+ len 4))])))

; We can be pretty sure that the number with the max length will be odd
(defn solve (from (max-n 0) (max-len 0))
      (if (> from 1000000)
          max-n
          (let ([cur-len (collatz-length from)])
            (vector-push! known cur-len)
            (if (> cur-len max-len)
              (solve (inc from) from cur-len)
              (solve (inc from) max-n max-len)))))

(solution (solve 3))

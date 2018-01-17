; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)
; * Improved to 3:10 by skipping a step for odd numbers
;   since 3n + 1 for odd n is always even
;
; * Improve to 2:30 by skipping multiple steps
;   switching on n % 4
; * Improve to 2:20 by using func refs instead of rc lambdas
; * Improve to 1:40 by removing some `.clone()`s
; * Improve to 20s via new `vector-ref`
(def known '(0 1 2))
(def known-len 2)

(defn collatz-length (n (len 1))
  (if (<= n known-len)
      (+ len (list-ref known n))
      (let (rest (% n 4))
      (cond
        (= rest 0) (collatz-length (div n 4) (+ len 2))
        (= rest 1) (collatz-length (+ (* 3 (div n 4)) 1) (+ len 3))
        (= rest 2) (collatz-length (+ (* 3 (div n 4)) 2) (+ len 3))
        (= rest 3) (collatz-length (+ (* 9 (div n 4)) 8) (+ len 4))))))

; We can be pretty sure that the number with the max length will be odd
(defn solve (from (max-n 0) (max-len 0))
      (println "from = " from)
      (if (> from 1000000)
          max-n
          (let (cur-len (collatz-length from))
            (push! known cur-len)
            (set! known-len (inc known-len))
            (if (> cur-len max-len)
              (solve (inc from) from cur-len)
              (solve (inc from) max-n max-len)))))

(println "Solution: " (solve 3))

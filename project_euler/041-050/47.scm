; Solved 8.1
;
; First version: 85s
; Improved to 2.6s by checking against a list of 10k primes
; first in `prime-factors`

(defn solve ((cur 1) (streak 0))
  (println "cur = " cur)
  (cond
    (= streak 4) (- cur 4)
    (>= (length (prime-factors cur)) 4)
      (solve (inc cur) (inc streak))
    else (solve (inc cur) 0)))

(println "Solution: " (solve))

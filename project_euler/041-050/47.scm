; Solved 8.1
;
; First version: 85s
; Improved to 2.6s by checking against a list of 10k primes
; first in `prime-factors`

(defn search (cur streak)
  (println cur)
  (cond
    ((= streak 4) (- cur 4))
    ((>= (length (prime-factors cur)) 4)
     (search (inc cur) (inc streak)))
    (else (search (inc cur) 0))))

(println (search 1 0))

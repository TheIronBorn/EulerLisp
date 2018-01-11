; Solved 1.1
; Changes:
;  9.1, use bignum

(defn double (remaining acc)
  (if (zero? remaining)
      acc
      (double (dec remaining) (bignum+ acc acc))))

(println "Solution: " (sum (bignum-digits (double 1000 (bignum 1)))))

; Solved 1.1
; Changes:
;  9.1, use bignum

(~>
  (bigpow (bignum 2) 1000)
  bignum-digits
  sum
  (println "Solution: "))

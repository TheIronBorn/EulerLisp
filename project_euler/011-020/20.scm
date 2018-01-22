; Solved 1.1
; Changed 5.1, use bignum

(~> 
  (range 1 100)
  (reduce &(bignum* (bignum &1) &2) (bignum 1))
  bignum-digits
  sum
  (println "Solution: "))

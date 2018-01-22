; Solved 5.1
; Changes: Add bignum multiplication

(~>
  (range~ 1 1000)
  (map~ &(bigpow (bignum &1) &1))
  (reduce~ bignum+ (bignum 0))
  bignum-digits
  reverse
  (take 10)
  digits->number
  (println "Solution: "))

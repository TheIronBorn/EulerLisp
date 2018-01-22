; Solved 5.1
; Changes: Add bignum multiplication

(def candidates (range 2 100))
(def result
     (flatmap 
       (fn (a) (map &(bigpow (bignum a) &1) candidates))
       candidates))

(~> result sort uniq length (println "Solution: "))

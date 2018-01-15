; Solved 5.1
; Changes: Add bignum multiplication

(def candidates (range 2 100))
(def result
     (flatmap 
       (fn (a) (map (fn (b) (bigpow (bignum a) b)) candidates))
       candidates))

(~> result sort uniq length (println "Solution: "))

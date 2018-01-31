; Solved: 17.12.17
; Improved: 5.12.17, use bignum

(defn solve (a b (n 1))
      (if (>= (bignum-num-digits a) 1000)
          n
          (solve b (bignum+ a b) (inc n))))

(println "Solution: " (solve (bignum 1) (bignum 1)))

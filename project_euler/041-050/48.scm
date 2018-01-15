; Solved 5.1
; Changes: Add bignum multiplication

(defn solve (from result)
      (println "from = " from)
      (if (> from 1000)
          result
          (solve
            (inc from)
            (bignum+ result (bigpow (bignum from) from)))))

(~> (solve 1 (bignum 0))
    bignum-digits
    reverse
    (take 10)
    reverse
    (apply str)
    (println "Solution: "))

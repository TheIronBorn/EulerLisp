; Solved 1.1
; Changed 5.1, use bignum

(defn multiply-n (n acc)
      (if (= n 0)
          acc
          (multiply-n (dec n)
                      (bignum* acc (bignum n)))))

(~> (multiply-n 100 (bignum 1))
    bignum-digits
    sum
    (println "Solution: "))

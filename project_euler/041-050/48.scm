; Solved 5.1
; Changes: Add bignum multiplication

(defn loop (from to result)
      (println from)
      (if (> from to)
          result
          (loop (inc from) to
                (bignum+ result (bigpow (bignum from) from)))))

(~> (loop 1 1000 (bignum 0))
    bignum-digits
    reverse
    (take 10)
    reverse
    (apply str)
    println)

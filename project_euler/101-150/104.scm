; Solved: 27.1.18
; Time: 48.48s

(defn pandigital? (n)
      (~>
        n
        bignum-chunks
        fst
        number->digits
        (take 9)
        sort
        (= (range 1 9))))

(defn pandigital2? (n)
      (~>
        n
        bignum-digits
        reverse
        (take 9)
        sort
        (= (range 1 9))))

(defn fib (n a b)
      (if (and (pandigital? a) (pandigital2? a))
          n
          (fib (inc n) b (bignum+ a b))))

(println "Solution: " (fib 1 (bignum 1) (bignum 1)))

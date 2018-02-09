; Solved: 27.1.2018
; Time: 48.48s
; TODO: Remove bignum-chunks when % works on bignums

(def all-digits (range 1 9))

(defn pandigital? (n)
      (~>
        n
        bignum-chunks
        fst
        number->digits
        (take 9)
        sort
        (= all-digits)))

(defn pandigital2? (n)
      (~>
        n
        number->digits
        reverse
        (take 9)
        sort
        (= all-digits)))

(defn fib (n a b)
      (if (and (pandigital? a) (pandigital2? a))
          n
          (fib (inc n) b (+ a b))))

(solution (fib 1 (bignum 1) (bignum 1)))

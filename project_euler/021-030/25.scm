; Solved: 17.12.17
; Improved: 5.12.17, use bignum

(defn fib (n a b)
      (if (= 0 n)
          a
          (fib (dec n) b (bignum+ a b))))

(def a (bignum 1))
(def b (bignum 1))

(defn solve ((n 1))
      (if (>= (bignum-num-digits a) 1000)
          n
          (do
             (let (new_a b
                   new_b (bignum+ a b))
                  (set! a new_a)
                  (set! b new_b))
             (solve (inc n)))))

(println "Solution: " (solve))

; Solved: 17.12.17
; Improved: 5.12.17, use bignum

(defn fib (n a b)
      (if (= 0 n)
          a
          (fib (dec n) b (bg+ a b))))

(defn for (from to fun)
      (if (<= from to)
        (do (fun from)
            (for (inc from) to fun))))


(def a (bignum 1))
(def b (bignum 1))

(defn solve (n)
      (if (>= (bignum-digits a) 1000)
          (println n)
          (do
             (let ((new_a b) (new_b (bg+ a b)))
                  (set! a new_a)
                  (set! b new_b))
             (solve (inc n)))))

(solve 1)

; Solved: 22.12.17
; Builtin Deterministic miller-rabin

(defn nth-prime (n) (nth-prime_ (- n 2) 3))
(defn nth-prime_ (n cur)
      (println n)
      (if (prime? cur)
          (if (zero? n)
              cur
              (nth-prime_ (dec n) (+ cur 2)))
          (nth-prime_ n (+ cur 2))))

(println (nth-prime 10001))

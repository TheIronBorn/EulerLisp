; Solved: 29.1.18
; Time: x3:19
;        1:50 using phi in rust

(def n 10_000_000)

(defn permutation? (a)
      (let ([ds1 (number->digits (fst a))]
            [ds2 (number->digits (rst a))])
          (= (sort ds1) (sort ds2))))

; Skip over even numbers,
; because phi(2m) = 2phi(m) or phi(m),
; so 2m / 2phi(m) >= m / phi(m)
;
; phi(prime) = prime - 1 can never be a permutation
(~>
  (range~ 3 n 2)
  (select~ &(not (prime? &1)))
  (map~ &(cons &1 (totient &1)))
  (select~ permutation?)
  collect
  (min-by (fn (c) (/ (fst c) (rst c))))
  (println "Solution: "))

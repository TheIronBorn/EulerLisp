(defn square (x) (* x x))

(defn bigpow (b e)
  (cond
    [(= e 0) (bignum 1)]
    [(even? e) (bigpow (bignum* b b) (div e 2))]
    [else (bignum* b (bigpow b (dec e)))]))

; (defn fac (n) (product~ (range~ 1 n)))
(defn fac (n (acc 1))
  (if (zero? n) acc
      (fac (dec n) (* acc n))))

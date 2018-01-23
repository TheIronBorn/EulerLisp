(defn square (x) (* x x))
(defn pow (number exponent)
  (cond
    [(= exponent 0) 1]
    [(even? exponent) (pow (square number) (div exponent 2))]
    [else (* number (pow number (dec exponent)))]))

(defn bigpow (b e)
  (cond
    [(= e 0) (bignum 1)]
    [(even? e) (bigpow (bignum* b b) (div e 2))]
    [else (bignum* b (bigpow b (dec e)))]))

; (defn fac (n) (product~ (range~ 1 n)))
(defn fac (n (acc 1))
  (if (zero? n) acc
      (fac (dec n) (* acc n))))

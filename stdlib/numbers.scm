(defn square (x) (* x x))
(defn pow (number exponent)
  (cond
    (= exponent 0) 1
    (even? exponent) (pow (square number) (/ exponent 2))
    else (* number (pow number (dec exponent)))))

(defn bigpow (b e)
  (cond
    (= e 0) (bignum 1)
    (even? e) (bigpow (bignum* b b) (/ e 2))
    else (bignum* b (bigpow b (dec e)))))

(defn fac (n) (fac_ n 1))

(defn fac_ (n acc)
  (if (zero? n) acc
      (fac_ (dec n) (* acc n))))

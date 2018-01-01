(defn square (x) (* x x))
(defn pow (number exponent)
  (cond
    ((= exponent 0) 1)
    ((even? exponent) (pow (square number) (/ exponent 2)))
    (else
      (* number (pow number (dec exponent))))))

(defn fac (n) (fac_ n 1))

(defn fac_ (n acc)
  (if (zero? n) acc
      (fac_ (dec n) (* acc n))))

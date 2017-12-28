(defn square (x) (* x x))
(defn pow (number exponent)
  (cond
    ((= exponent 0) 1)
    ((even? exponent) (pow (square number) (/ exponent 2)))
    (else
      (* number (pow number (dec exponent))))))

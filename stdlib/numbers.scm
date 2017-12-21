(defn dec (n) (- n 1))
(defn inc (n) (+ n 1))

(defn zero? (n) (= 0 n))
(defn divides? (n m) (= 0 (% m n)))
(defn even? (n) (divides? 2 n))
(defn odd? (n) (not (divides? 2 n)))

(defn max (a b) (if (> a b) a b))
(defn min (a b) (if (< a b) a b))

(defn square (x) (* x x))
(defn pow (number exponent)
  (cond
    ((= exponent 0) 1)
    ((even? exponent) (pow (square number) (/ exponent 2)))
    (else
      (* number (pow number (dec exponent))))))

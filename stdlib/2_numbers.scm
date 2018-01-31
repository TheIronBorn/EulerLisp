(defn inc (n) (+ n 1))
(defn dec (n) (- n 1))

(defn square (x) (* x x))

(defn pow (b e)
  (cond
    [(= e 0) 1]
    [(even? e) (pow (* b b) (div e 2))]
    [else (* b (pow b (dec e)))]))

; (defn fac (n) (product~ (range~ 1 n)))
(defn fac (n (acc 1))
  (if (zero? n) acc
      (fac (dec n) (* acc n))))

(defn abs (x) (if (< x 0) (- x) x))
(defn isqrt (n) (floor (sqrt n)))

(defn gauss-sum (n) (div (* n (inc n)) 2))
(defn gauss-square-sum (n) (div (* n (inc n) (inc (* 2 n))) 6))

(defn binomial (n k)
  (div (fac n) (* (fac k) (fac (- n k)))))

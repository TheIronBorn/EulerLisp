(defn inc (n) (+ n 1))
(defn dec (n) (- n 1))

(defn square (x) (* x x))
(defn cube (x) (* x x x))

(defn pow (b e)
  (cond
    [(= e 0) 1]
    [(even? e) (pow (* b b) (div e 2))]
    [else (* b (pow b (dec e)))]))

(defn fac (n)
  (defn inner (n acc)
    (if (zero? n) acc
        (inner (dec n) (* acc n))))
  (inner n 1))

(defn abs (x)
  (if (< x 0) (- x) x))
(defn isqrt (n)
  (floor (sqrt n)))
(defn icbrt (n)
  (floor (cbrt n)))
(defn square? (n)
  (= (square (isqrt n)) n))

(defn gauss-sum (n)
  (div (* n (inc n)) 2))
(defn gauss-square-sum (n)
  (div (* n (inc n) (inc (* 2 n))) 6))

(defn solve-quadratic (a b c)
  (let* ([det (- (square b) (* 4 a c))])
    (cond
      [(< det 0) (list)]
      [(= det 0) (list (/ (- b) (* 2 a)))]
      [else
        (let ([r (sqrt det)])
          (list
            (/ (- (- b) r) (* 2 a))
            (/ (+ (- b) r) (* 2 a))))])))

(defn product (from to fun)
  (defn inner (cur acc)
    (if {cur > to}
        acc
        (inner (inc cur)
               (* acc (fun cur)))))
  (inner from 1))

(defn sum (from to fun)
  (defn inner (cur acc)
    (if {cur > to}
        acc
        (inner (inc cur)
               (+ acc (fun cur)))))
  (inner from 0))

(defn id (x) x)

(defn fac-from (from to)
  (product from to id))

(defn binomial (n r)
  (let ([other (max r (- n r))])
    (div (fac-from (+ other 1) n)
         (fac-from 1 (- n other)))))

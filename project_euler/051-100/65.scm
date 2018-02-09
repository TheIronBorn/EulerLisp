; Solved 26.1.2018

(defn cont (n)
  (if (= 1 (% n 3))
      (* 2 (/ (+ n 2) 3))
      1))

(defn frac+ (a b)
  (cons
    (+ (* (fst a) (rst b))
       (* (fst b) (rst a)))
    (* (rst a) (rst b))))

(defn frac* (a b)
  (cons
      (* (fst a) (fst b))
      (* (rst b) (rst a))))

(defn frac-invert (a)
  (cons (rst a) (fst a)))

(defn integer->frac (n) (cons n 1))

(defn convergent_ (n acc)
  (if (= n -1)
      (frac+ (integer->frac 2) acc)
      (convergent_ (dec n)
                   (frac-invert
                     (frac+ acc (integer->frac (cont n)))))))

(defn convergent (a) (convergent_ a (integer->frac 0)))

(~>
  (- 100 2)
  convergent
  fst
  number->digits
  sum
  solution)

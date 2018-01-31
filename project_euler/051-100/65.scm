; Solved 26.1.18

(defn cont (n)
  (if (= 1 (% n 3))
      (* 2 (/ (+ n 2) 3))
      1))

(defn bigfrac+ (a b)
  (cons
    (bignum+
      (bignum* (fst a) (rst b))
      (bignum* (fst b) (rst a)))
    (bignum* (rst a) (rst b))))

(defn bigfrac* (a b)
  (cons
      (bignum* (fst a) (fst b))
      (bignum* (rst b) (rst a))))

(defn bigfrac-invert (a)
  (cons (rst a) (fst a)))

(defn integer->bigfrac (n)
      (cons (bignum n) (bignum 1)))

(defn convergent_ (n acc)
  (if (= n -1)
      (bigfrac+ (integer->bigfrac 2) acc)
      (convergent_ (dec n)
                   (bigfrac-invert
                     (bigfrac+ acc (integer->bigfrac (cont n)))))))

(defn convergent (a)
  (convergent_ a (integer->bigfrac 0)))

(~>
  (- 100 2)
  convergent
  fst
  bignum-digits
  sum
  (println "Solution: "))

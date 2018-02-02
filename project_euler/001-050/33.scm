; Solved 6.1
; We can't use the builtin fractions here,
; because they are simplified on creation

(defn fraction= (a b)
  (= (* (fst a) (rst b))
     (* (rst a) (fst b))))

(defn digit-cancelling? (f)
  (let ([a (div (fst f) 10)]
        [b (%   (fst f) 10)]
        [c (div (rst f) 10)]
        [d (%   (rst f) 10)])
    (or
      (and (= a d)
           (fraction= f (cons b c)))
      (and (= b c)
           (fraction= f (cons a d)))
      (and (= a c)
           (fraction= f (cons b d)))
      (and (= b d)
           (!= b 0)
           (fraction= f (cons a c))))))

(defn solve (d n (acc '()))
  (cond
    (>= n d) (solve (inc d) 10 acc)
    (> d 99) acc
    (digit-cancelling? (cons n d))
      (solve d (inc n) (cons (cons n d) acc))
    else
      (solve d (inc n) acc)))

(defn fraction* (a b)
  (cons
    (* (fst a) (fst b))
    (* (rst a) (rst b))))

(defn simplify (f)
  (let ([g (gcd (fst f) (rst f))])
    (cons (div (fst f) g)
          (div (rst f) g))))

(~>
  (solve 11 10)
  (reduce fraction* (cons 1 1))
  simplify
  rst
  (println "Solution: "))
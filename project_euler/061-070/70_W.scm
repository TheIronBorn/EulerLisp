; Solved: 27.1.18

(defn phi_ (n)
  (~>
    (range~ 1 (dec n))
    (count~ &(= 1 (gcd n &1)))))

(defn phi (n)
  (* n (reduce-product &(- 1 (/ 1 (fst &1))) (prime-factors n))))

(def phis
     (~>
       (range~ 2 (dec 10000000))
       (map~ &(cons &1 (phi &1)))
       (select~ (fn (a)
                    (= (sort (number->digits (fst a)))
                       (sort (number->digits (rst a))))))
       collect
       ))

(~>
  phis
  (min-by (fn (p) (/ (fst p) (rst p))))
  (println "Solution: "))

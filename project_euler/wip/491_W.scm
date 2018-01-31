; Solved:

(def ds
     (list 4 0 5 6 1 8 1 7 7 0 3 8 2 3 5 6 4 9 2 9))

(defn alternating-digit-sum (ds (acc 0) (sign #t))
  (if (nil? ds)
      acc
      (alternating-digit-sum (rst ds)
                             (if sign
                               (- acc (fst ds))
                               (+ acc (fst ds)))
                             (not sign))))

(println (alternating-digit-sum ds))

(defn div-11? (ds)
  (divides? 11 (alternating-digit-sum ds)))


(~>
  (list 0 1 2 3 4 5 6 7 8)
  permutations~
  (select~ &(!= 0 (fst &1)))
  (count~ div-11?)
  (println "Solution: "))

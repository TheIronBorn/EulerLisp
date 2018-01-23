; Solved: 13.1.18

(defn triangle (n)
      (div (* n (inc n)) 2))

; Use the same idea as for problem 44
(defn pentagonal? (n)
      (let* (det (inc (* 24 n))
                 root (floor (isqrt det)))
        (and (= det (* root root))
             (divides? 6 (inc root)))))

(defn hexagonal? (n)
      (let* (det (inc (* 8 n))
                 root (floor (sqrt det)))
        (and (= det (* root root))
             (divides? 4 (inc root)))))

(~>
  (step~ (inc 285))
  (map~ triangle)
  (select~ &(and (pentagonal? &1) (hexagonal? &1)))
  first~
  (println "Solution: "))

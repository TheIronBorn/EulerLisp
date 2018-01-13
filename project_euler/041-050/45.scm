; Solved: 13.1.18

(defn triangle (n)
      (/ (* n (inc n)) 2))

; Use the same idea as for problem 44
(defn pentagonal? (n)
      (let* (det (inc (* 24 n))
                 root (isqrt det))
        (and (= det (* root root))
             (divides? 6 (inc root)))))

(defn hexagonal? (n)
      (let* (det (inc (* 8 n))
                 root (isqrt det))
        (and (= det (* root root))
             (divides? 4 (inc root)))))

(defn find-first (n)
      (let (x (triangle n))
        (if (and (pentagonal? x) (hexagonal? x))
          x
          (find-first (inc n)))))

; To be perfectly sure this is the right solution,
; we could prove that P_n grows in such a way,
; that we would return the solution with the smallest difference first
(println "Solution: " (find-first (inc 285)))

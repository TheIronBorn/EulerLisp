; Solved 8.1

(def names
     (~> "project_euler/041-050/42.txt"
         file-read
         (string-split ",")
         sort))

(defn value (name)
      (sum
        (map (fn (x)
                 (if (and (>= x 65) (<= x 90))
                     (- x 64)
                     0))
          (string-bytes name))))

(defn triangle? (n)
      (triangle?_ n 2 1))

(defn triangle?_ (n m acc)
  (cond
    (< n acc) #f
    (= n acc) #t
    else (triangle?_ n (inc m) (+ m acc))))

(defn solve (names index acc)
      (if (empty? names)
          acc
          (if (~> names fst value triangle?)
              (solve
                (rst names)
                (inc index)
                (inc acc))
              (solve
                (rst names)
                (inc index)
                acc)
            )))

(println "Solution: " (solve names 1 0))

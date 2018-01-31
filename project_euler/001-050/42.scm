; Solved 8.1

(defn value (name)
      (reduce-sum
        (fn (c)
            (if (char-alphabetic? c)
              (- (char->integer c) 64)
              0))
        (string->chars name)))

(defn triangle? (n (m 2) (acc 1))
  (cond
    [(< n acc) #f]
    [(= n acc) #t]
    [else (triangle? n (inc m) (+ m acc))]))

(defn solve (names (index 1) (acc 0))
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

(def names
     (~> "project_euler/input-files/42.txt"
         file-read
         (string-split ",")
         sort
         solve
         (println "Solution: ")))

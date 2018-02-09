; Solved 8.1.2018

(defn value (name)
      (reduce-sum
        (fn (c)
            (if (char-alphabetic? c)
              (- (char->integer c) 64)
              0))
        (string->chars name)))

(defn solve (names (index 1) (acc 0))
      (if (empty? names)
          acc
          (if (~> names fst value triangular?)
              (solve
                (rst names)
                (inc index)
                (inc acc))
              (solve
                (rst names)
                (inc index)
                acc))))

(~> "project_euler/input-files/42.txt"
   file-read
   (string-split ",")
   sort
   solve
   solution)

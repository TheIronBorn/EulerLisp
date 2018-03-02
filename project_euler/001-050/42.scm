; Solved 8.1.2018

(defn value (name)
      (reduce-sum
        (fn (c)
            (if (char-alphabetic? c)
              (- (char->integer c) 64)
              0))
        (string->chars name)))

(defn solve (names)
  (defn inner (names index acc)
      (if (empty? names)
          acc
          (if (~> names fst value triangular?)
              (inner
                (rst names)
                (inc index)
                (inc acc))
              (inner
                (rst names)
                (inc index)
                acc))))
  (inner names 1 0))

(~> "project_euler/input-files/42.txt"
   file-read
   (string-split ",")
   sort
   solve
   solution)

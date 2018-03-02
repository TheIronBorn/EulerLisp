; Solved 1.1.2018

(defn value (name)
      (reduce-sum
        &(if (char-alphabetic? &1) (- (char->integer &1) 64) 0)
        (string->chars name)))

(defn solve (names)
  (defn inner (names index acc)
      (if (nil? names)
        acc
        (inner
          (rst names)
          (inc index)
          (+ acc (* index (value (fst names)))))))
  (inner names 1 0))

(~> "project_euler/input-files/22.txt"
    file-read
    (string-split ",")
    (reject string-empty?)
    sort
    solve
    solution)

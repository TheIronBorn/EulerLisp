; Solved 1.1

(def input
  (map (fn (x)
           (map string->number (words x)))
      (rst (reverse (lines (file-read "project_euler/67.txt"))))))

(defn reduce-rows (a b acc)
  (if (nil? b)
      (reverse acc)
      (reduce-rows
        (rst a)
        (rst b)
        (cons
          (max
            (+ (fst a) (fst b))
            (+ (frst a) (fst b)))
          acc))))

(defn reduce-full (input)
    (if (= 1 (length input))
        (ffst input)
        (reduce-full
          (cons
            (reduce-rows (fst input) (frst input) '())
            (rrst input)))))

(println (reduce-full input))

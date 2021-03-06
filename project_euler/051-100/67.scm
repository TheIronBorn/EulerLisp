; Solved 1.1.2018

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

(~>
  (file-read "./project_euler/input-files/67.txt")
  lines
  reverse
  rst
  (map &(map string->number (words &1)))
  reduce-full
  solution)

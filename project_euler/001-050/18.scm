; Solved 1.1.2018

(defn reduce-rows (a b)
  (defn inner (a b acc)
    (if (nil? b)
      (reverse acc)
      (inner (rst a) (rst b)
             (cons
               (max
                 (+ (fst a) (fst b))
                 (+ (frst a) (fst b)))
               acc))))
  (inner a b '()))

(defn reduce-full (input)
    (if (= 1 (length input))
        (ffst input)
        (reduce-full
          (cons
            (reduce-rows (fst input) (frst input))
            (rrst input)))))

(~>
  "project_euler/input-files/18.txt"
  file-read
  lines
  reverse
  rst
  (map &(map string->number (words &1)))
  reduce-full
  solution)

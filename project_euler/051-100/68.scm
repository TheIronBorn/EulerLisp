; Solved: 27.1.18

(defn to-ring (p)
  (list
    (list 10 (nth 4 p) (nth 5 p))
    (list (nth 0 p) (nth 5 p) (nth 6 p))
    (list (nth 1 p) (nth 6 p) (nth 7 p))
    (list (nth 2 p) (nth 7 p) (nth 8 p))
    (list (nth 3 p) (nth 8 p) (nth 4 p))))

(defn rotate-ring_ (r m)
  (if (= (ffst r) m)
      r
      (rotate-ring_
        (push (rst r) (fst r))
        m)))

(defn rotate-ring (r)
  (rotate-ring_ r (reduce-min fst 11 r)))

(defn flatten (a) (flatmap id a))

(~>
  (permutations~ (range 1 9))
  (select~ (fn (p)
               (let ([s (+ 10 (nth 4 p) (nth 5 p))])
                 (and
                   (= s (+ (nth 0 p) (nth 5 p) (nth 6 p)))
                   (= s (+ (nth 1 p) (nth 6 p) (nth 7 p)))
                   (= s (+ (nth 2 p) (nth 7 p) (nth 8 p)))
                   (= s (+ (nth 3 p) (nth 8 p) (nth 4 p)))))))
  (map~ to-ring)
  (map~ rotate-ring)
  (map~ flatten)
  (map~ &(apply str &1))
  collect
  (apply max)
  println)


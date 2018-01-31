; Solved: 26.1.18

(def input
     (~>
       "./project_euler/input-files/81.txt"
       file-read
       lines
       (reject empty?)
       (map &(map string->number (string-split "," &1)))))

(defn reduce-rows (a b (last '()) (acc '()))
      (if (nil? a)
        acc
        (let ([above (fst a)]
              [cur (fst b)])
          (let ([best (if (nil? last)
                          (+ above cur)
                          (min (+ last cur) (+ above cur)))])
            (reduce-rows
              (rst a)
              (rst b)
              best
              (push acc best))))))

(defn reduce-first (a (last 0) (acc '()))
      (if (nil? a)
        acc
        (reduce-first
          (rst a)
          (+ last (fst a))
          (push acc (+ last (fst a))))))

(defn reduce-full_ (m last)
    (if (nil? m)
      last
      (reduce-full_
        (rst m)
        (reduce-rows last (fst m)))))

(defn reduce-full (m)
      (reduce-full_
        (rst m)
        (reduce-first (fst m))))

(println "Solution: " (last (reduce-full input)))

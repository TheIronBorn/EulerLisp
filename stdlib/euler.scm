(defn solution (s) (println "Solution: " s))

(defn integral? (x) (= (floor x) (ceil x)))

(defn triangular (n) (div (* n (+ n 1)) 2))
(defn pentagonal (n) (div (* n (- (* 3 n) 1)) 2))
(defn hexagonal  (n) (* n (- (* 2 n) 1))) 
(defn heptagonal (n) (div (* n (- (* 5 n) 3)) 2))
(defn octagonal  (n) (* n (- (* 3 n) 2))) 

(defn triangular? (n)
  (let ([solutions (solve-quadratic 1 -1 (- {2 * n}))])
    (and (not (empty? solutions))
         (integral? (frst solutions)))))

(defn pentagonal? (n)
  (let ([solutions (solve-quadratic 3 -1 (- {2 * n}))])
    (and (not (empty? solutions))
         (integral? (frst solutions)))))

(defn hexagonal? (n)
  (let ([solutions (solve-quadratic 2 -1 (- n))])
    (and (not (empty? solutions))
         (integral? (frst solutions)))))

(defn input-file-lines (file)
     (~> file
         file-read
         lines
         (reject &(= "" &1))))

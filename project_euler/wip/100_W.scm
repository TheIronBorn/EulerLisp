; Solved: 

(defn find-b (n)
  (div (+ 1 (isqrt (+ (* 2 n n) (* -2 n) 1))) 2))

(defn solves? (n b)
  (= (* 2 b (- b 1))
     (* n (- n 1))))

(defn loop (from)
  (let ([b (find-b (number->float from))])
    (cond
      [(solves? from b) (println "n: " from " b: " b)]
      [(solves? from (inc b)) (println "n: " from " b: " (inc b))]
      [else (loop (inc from))])))

(def from (pow 10 12))
(loop from)

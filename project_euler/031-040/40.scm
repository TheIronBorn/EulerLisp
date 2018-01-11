; Solved 9.1

(def ranges
   (map (fn (x) (* x 9 (pow 10 (dec x))))
        (range 1 7)))

(defn find-bracket (n)
      (find-bracket_ n 1 ranges))

(defn find-bracket_ (n cur ranges)
  (if (< n (fst ranges))
      (cons cur n)
      (find-bracket_ (- n (fst ranges)) (inc cur) (rst ranges))))

(defn find-digit (range rest)
    (let* (real-rest (+ rest (* range (pow 10 (dec range))))
           position (dec (- range (% real-rest range))))
         (% (/ (/ real-rest range) (pow 10 position)) 10)))

(defn digit (n)
  (if (<= n 9)
      n
      (let (bracket (find-bracket (dec n)))
        (find-digit (fst bracket) (rst bracket))
        )))

(println "Solution: " (product (map (fn (n) (digit (pow 10 n))) (range 0 6))))

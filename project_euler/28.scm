; Solved 1.1
; Idea: See notebook

(defn ring-sum (i)
    (if (zero? i)
        1
        (- (* 4 (square (inc (* 2 i))))
           (* i 12))))

(defn solve (n acc size)
      (if (= n size)
          acc
          (solve (inc n)
                 (+ acc (ring-sum n))
                 size)))

(def n 1001)
(def size (inc (/ n 2)))

(println (solve 0 0 size))

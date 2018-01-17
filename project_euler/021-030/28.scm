; Solved 1.1
; Idea: See notebook

(defn ring-sum (i)
    (if (zero? i)
        1
        (- (* 4 (square (inc (* 2 i))))
           (* i 12))))

(defn solve (n size (acc 0))
      (if (= n size)
          acc
          (solve (inc n)
                 size
                 (+ acc (ring-sum n)))))

(println "Solution: " (solve 0 (inc (div 1001 2))))

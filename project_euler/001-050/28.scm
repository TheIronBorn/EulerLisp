; Solved 1.1.2018

(defn ring-sum (i)
    (if (zero? i)
        1
        (- (* 4 (square (inc (* 2 i))))
           (* i 12))))

(~>
  (range~ 0 (div 1001 2))
  (map~ ring-sum)
  sum~
  solution)

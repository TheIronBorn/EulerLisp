; Solved: 26.1.2018

; Just work with the transitions saved each step
; instead of caluclating the counts of sam and max
; and subtracting them
(def transitions-saved
     (list
       (list 12  4  8  8  6  8 10  8 12 10)
       (list  4  4  2  4  4  2  2  4  4  4)
       (list  8  2 10  8  4  6  8  4 10  8)
       (list  8  4  8 10  6  8  8  6 10 10)
       (list  6  4  4  6  8  6  6  6  8  8)
       (list  8  2  6  8  6 10 10  6 10 10)
       (list 10  2  8  8  6 10 12  6 12 10)
       (list  8  4  4  6  6  6  6  8  8  8)
       (list 12  4 10 10  8 10 12  8 14 12)
       (list 10  4  8 10  6  8  8  8 12 12)))

(defn count-saved (a b (acc 0))
      (if (or (zero? a) (zero? b))
          acc
          (count-saved
            (div a 10) (div b 10)
            (+ acc
               (~> transitions-saved (nth (% b 10)) (nth (% a 10)))))))

(defn process-saved (n (acc 0))
  (if (< n 10)
      acc
      (let ([ds (sum (number->digits n))])
        (process-saved
          ds
          (+ acc (count-saved n ds))))))

(~>
  (range~ 10000001 19999999 2)
  (select~ prime?)
  (map~ process-saved)
  sum~
  solution)

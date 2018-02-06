; Solved _
; Note:
; * https://en.wikipedia.org/wiki/Pell%27s_equation#The_smallest_solution_of_Pell_equations
; * Implement expansion of continued fractions of square roots,
;   this is needed for a few other problems, too

(defn square? (n)
  (= n
     (square (floor (sqrt n)))))

(defn find-x (d (x 2))
  (println d " " x)
  (let ([x_ (dec (square x))])
    (if (and (divides? d x_)
             (square? (div x_ d)))
      x
      (find-x d (inc x)))))

(~>
  (range~ 1 70)
  (select~ &(not (square? &1)))
  collect
  (max-by find-x)
  println
  )

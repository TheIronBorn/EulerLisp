; Solved: 17.12.17

(def square_of_sums (square (sum (range 1 100))))
(def sum_of_squares (sum (map square (range 1 100))))

(println (- square_of_sums sum_of_squares))

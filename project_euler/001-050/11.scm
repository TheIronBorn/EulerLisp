; Solved: 29.12.2017

(def input (input-file-lines "project_euler/input-files/11.txt"))

(def grid
  (map &(map string->number (words &1)) input))

(defn diagonals (rows)
    (transpose
      (list
        (fst rows)
        (rst (frst rows))
        (rrst (frrst rows))
        (rrrst (frrrst rows)))))

(def row-chunks (flatmap &(chunks 4 &1) grid))
(def col-chunks (flatmap &(chunks 4 &1) (transpose grid)))
(def diagonal-chunks (flatmap diagonals (chunks 4 grid)))
(def antidiagonal-chunks (flatmap diagonals (chunks 4 (map reverse grid))))

(defn max-product (lst) (apply max (map product lst)))

(solution
  (max
    (max-product row-chunks)
    (max-product col-chunks)
    (max-product diagonal-chunks)
    (max-product antidiagonal-chunks)))

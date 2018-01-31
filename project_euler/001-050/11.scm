; Solved: 29.12.17
; Changes:
;  * add `any?`, `all?`, `zip`, `sum`, `product`, `take`, `chunks` list functions
;  * add `curry` fn functions

(def input
     (~> "project_euler/input-files/11.txt"
         file-read
         lines
         (reject empty?)))

(def chunks4 (curry chunks 4))

(def grid
  (map &(map string->number (words &1)) input))

(defn transpose (lst)
      (map
        &(map (curry nth &1) lst)
        (range 0 (dec (length (fst lst))))))

(defn diagonals (rows)
      (zip
        (fst rows)
        (rst (frst rows))
        (rrst (frrst rows))
        (rrrst (frrrst rows))))

(def row-chunks (flatmap chunks4 grid))
(def col-chunks (flatmap chunks4 (transpose grid)))
(def diagonal-chunks (flatmap diagonals (chunks4 grid)))
(def antidiagonal-chunks (flatmap diagonals (chunks4 (map reverse grid))))

(defn max-product (lst)
      (apply max (map product lst)))

(println
  (max
    (max-product row-chunks)
    (max-product col-chunks)
    (max-product diagonal-chunks)
    (max-product antidiagonal-chunks)))

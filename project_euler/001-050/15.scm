; Solved 1.1.2018

(def n 20)

(~>
  (range~ 1 n)
  (map~ &(/ (+ n &1) &1))
  product~
  solution)

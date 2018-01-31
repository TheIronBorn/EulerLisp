; Solved 1.1,
; changed to use binomials on 31.1.18

(def n 20)
(~>
  (range~ 1 n)
  (map~ &(/ (+ n &1) &1))
  (reduce~ * 1)
  solution)

; Solved: 13.1.2018

(~>
  (step~ (inc 285))
  (map~ triangular)
  (select~ &(and (pentagonal? &1) (hexagonal? &1)))
  first~
  solution)

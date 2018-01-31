; Solved 5.1

(~>
  (combinations~ 2 (range 2 100))
  (map~ &(pow (fst &1) (frst &1)))
  collect
  sort
  uniq
  length
  solution)

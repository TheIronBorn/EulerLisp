; Solved 11.1.2018

(~>
  (combinations~ 2 (range 1 99))
  (map~ &(~> (frst &1)
             (pow (fst &1))
             number->digits
             list-sum))
  (reduce~ max 0)
  solution)

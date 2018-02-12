; Solved: 29.12.2017

(~>
  (step~ 1)
  (accumulate~ + 0)
  (select~ &(~> &1 factors length (< 500)))
  first~
  solution)

; Solved: 

(defn solutions (n)
  (~>
    (range~ (inc n) (* 2 n))
    (count~ &(divides? {&1 - n} {n * &1}))))

(~>
  (step~ 4)
  (select~ &(> (solutions &1) 1000))
  first~
  solution)

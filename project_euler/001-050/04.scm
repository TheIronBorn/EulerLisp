; Solved: 17.12.2017

(defn loop (a)
  (~>
    (range~ a 999)
    (map~ &(* a &1))
    (select~ &(palindromic? (number->digits &1)))
    (reduce~ max 0)))

(~>
  (range~ 100 999)
  (map~ loop)
  (reduce~ max 0)
  solution)

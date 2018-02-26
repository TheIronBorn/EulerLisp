; Solved: 

(defn s (p)
  (+ p (% -45 p)))

(println (s 7))

(~>
  ; (range~ 5 100_000_000 2)
  (range~ 5 100 2)
  (select~ prime?)
  (map~ s)
  sum~
  solution)

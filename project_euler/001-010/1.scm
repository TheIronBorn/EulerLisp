; Solved: 17.12.17
; Changes: Switch to streams

(~>
  (range~ 1 1000)
  (select~ (fn (x) (or (divides? 3 x) (divides? 5 x))))
  sum~
  (println "Solution: "))

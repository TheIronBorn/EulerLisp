; Solved: 20.12.17
; Notes:
;  recursion w/ a variable for the known primes
;  takes up to much memory because environments are not freed
; Changes:
;  * add (rand low high) function
;  * add builtin (powmod base exponend mod) function
;  * add builtin bitshift
;  * switch to streams

(~>
  (step~ 3 2)
  (select~ prime?)
  (nth~ (- 10001 2))
  (println "Solution: "))

; Solved 23.1.18
; Time: 29:21
; Solution: 1739023853137

(defn prime-generating? (n)
    (all?
      (fn (x) (prime? (+ x (div n x))))
      (factors n)))

; All odd numbers
; split into factors that are odd
; (otherwise they would be even).
; The sum of two odd numbers can't be prime
; (unless the numbers are 1 and 1)
;
; If the number is divisible by 4,
; (n / 2) + 2 is even => not prime
; 
; Solution: Start at 2 and make steps of 4
; to save time
(~>
  (range~ 2 100000000 4)
  (select~ prime-generating?)
  sum~
  inc
  println)


; Solved 8.1.2018

(defn solve (n)
      (~>
        (range 1 n)
        permutations~
        (map~ digits->number)
        (select~ prime?)
        collect
        (reduce max 0)))

; 1 + ... + n = 
;   1 -> 1, trivial
;   2 -> 3, div by 3
;   3 -> 6, div by 3
;   4 -> 10
;   5 -> 15, div by 3
;   6 -> 21, div by 3
;   7 -> 28
;   8 -> 36, div by 3
;   9 -> 45, div by 3
;
; A number is divisible by 3
; if the sum of its digits is divisible by 3
; => can't be prime

(solution (max (solve 4) (solve 7)))

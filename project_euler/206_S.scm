; Solved 21.1.18
; Initial time: 9:34,
; reduced to 41s by making steps of 20 each time 

(def square-min
     (floor (sqrt 1020304050607080900)))
(def square-max
     (ceil (sqrt 1929394959697989990)))

; The only way for our square to end with a 0
; is to use a number that is a multiple of 10.
; This means, that the last 3 digits must be 900
; => number is a multiple of 30 or 70
;
; It is safe to make steps of 20
; 3 => 5 => 7 => 9 => 1 => 3
; and square-min is already a multiple of 30
(~> (range~ square-min square-max 20)
    (map~ square)
    (select~ &(= 9 (% (div &1 100) 10)))
    (select~ &(= 8 (% (div &1 10000) 10)))
    (select~ &(= 7 (% (div &1 1000000) 10)))
    (select~ &(= 6 (% (div &1 100000000) 10)))
    (select~ &(= 5 (% (div &1 10000000000) 10)))
    (select~ &(= 4 (% (div &1 1000000000000) 10)))
    (select~ &(= 3 (% (div &1 100000000000000) 10)))
    (select~ &(= 2 (% (div &1 10000000000000000) 10)))
    (select~ &(= 1 (% (div &1 1000000000000000000) 10)))
    first~
    sqrt
    floor
    (println "Solution: "))


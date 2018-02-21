; Solved 21.1.18

; The only way for our square to end with a 0
; is to use a number that is a multiple of 10.
; This means, that the last 3 digits must be 900
; => number ends with 30 or 70

(def square-min (floor (sqrt 1020304050607080900)))

(def square-min1 (+ 30 (* (div square-min 100) 100)))
(def square-min2 (+ 70 (* (div square-min 100) 100)))

(def square-max (ceil (sqrt 1929394959697989990)))

(defn valid? (n)
  (and (= 9 (% (div n 100) 10))
       (= 8 (% (div n 10000) 10))
       (= 7 (% (div n 1000000) 10))
       (= 6 (% (div n 100000000) 10))
       (= 5 (% (div n 10000000000) 10))
       (= 4 (% (div n 1000000000000) 10))
       (= 3 (% (div n 100000000000000) 10))
       (= 2 (% (div n 10000000000000000) 10))
       (= 1 (% (div n 1000000000000000000) 10))))

(def solution1
  (~> (range~ square-min1 square-max 100)
      (map~ square)
      (select~ valid?)
      first~))

(def solution2
  (~> (range~ square-min2 square-max 100)
      (map~ square)
      (select~ valid?)
      first~))

; There is only one solution
(~> (if (nil? solution1) solution2 solution1)
    isqrt
    solution)

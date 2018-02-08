; Solved: 6.2.2018

; The way floats work right now
; they are always different from integers,
; so comparing n to (floor n) is not enough
(defn integral? (n) (= (ceil n) (floor n)))

(defn integral-area? (a b)
    (integral? (* b (sqrt (- (square a) (square b))))))

(defn solve (a)
  (if (integral-area? a (/ (dec a) 2))
      (if (integral-area? a (/ (inc a) 2)) 2 1)
      (if (integral-area? a (/ (inc a) 2)) 1 0)))

; It's safe to start at 2,
; (1, 1, 0) and (1, 1, 2) don't have a integral area
(~>
  (range~ 2 1_000_000)
  (map~ solve)
  (sum~)
  solution)

; Solved: 21.2.2018

; A triangle as a right angle
; <=> if two of its sides are perpendicular
; <=> two sides have a dot-product of 0
(defn has-rigth-angle? (op oq)
  (let ([op-x (fst op)] [op-y (rst op)]
        [oq-x (fst oq)] [oq-y (rst oq)])
    (let ([pq-x (- oq-x op-x)]
          [pq-y (- oq-y op-y)])
      (or (zero? (+ (* op-x oq-x) (* op-y oq-y)))
          (zero? (+ (* op-x pq-x) (* op-y pq-y)))
          (zero? (+ (* oq-x pq-x) (* oq-y pq-y)))))))

(~>
  ; Create a list with all possible points (excluding O)
  (range 0 50)
  (combinations~ 2)
  (select~ &(not (= 0 (fst &1) (frst &1))))
  (map~ &(cons (fst &1) (frst &1)))
  collect
  ; Iterate over all possible pairs
  (combinations~ 2)
  ; Make sure to only count each triangle once
  (select~ &(> (fst &1) (frst &1)))
  (count~ &(has-rigth-angle? (fst &1) (frst &1)))
  solution)

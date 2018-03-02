; Solved: 22.12.2017

(defn prime-sum (limit)
  (defn inner (cur acc)
      (if (> cur limit)
          acc
          (if (prime? cur)
              (inner (+ cur 2) (+ acc cur))
              (inner (+ cur 2) acc))))
  (inner 3 2))

(solution (prime-sum 2_000_000))

; Implement take-while for streams
; (~>
;   (step~ 3 2)
;   (select~ prime?)
;   (take-while~ &(< &1 2_000_000))
;   sum~
;   solution)

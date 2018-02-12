; Solved: 22.12.2017

(defn prime-sum (limit (cur 3) (acc 2))
      (if (> cur limit)
          acc
          (if (prime? cur)
              (prime-sum limit (+ cur 2) (+ acc cur))
              (prime-sum limit (+ cur 2) acc))))

(solution (prime-sum 2_000_000))

; Implement take-while for streams
; (~>
;   (step~ 3 2)
;   (select~ prime?)
;   (take-while~ &(< &1 2_000_000))
;   sum~
;   solution)

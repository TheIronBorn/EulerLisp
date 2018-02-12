; Solved: 12.2.2018

; Factors
; 1. Offset from the start
; 2. Length of the first block

; Ways to fill a length
; so that the first block is not empty
(def known-filled #(0 0 0 1))

(defn filled (len)
  (if (< len (vector-length known-filled))
    (vector-ref known-filled len)
    (~>
      (range~ 3 len) ; Length of the first block
      (map~
        (fn (bl)
          (~>
            (range~ 1 {len - bl}) ; Offset to the next block
            (map~ &(filled {len - bl - &1}))
            sum~
            inc
          )))
      sum~)))

(~> (range~ 4 50)
    (each~
      &(vector-push! known-filled (filled &1))))

; All combinations = all possible offsets * filled combinations for this offset
(~> (range~ 3 50)
    (map~ filled)
    sum~
    inc
    solution)

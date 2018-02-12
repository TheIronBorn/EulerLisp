; Solved: 12.2.2018

(def m 50)

(def known-filled (make-vector m 0))
(vector-push! known-filled 1)

(defn filled (len)
  (if (< len (vector-length known-filled))
    (vector-ref known-filled len)
    (~>
      (range~ m len) ; Length of the first block
      (map~
        (fn (bl)
          (~>
            (range~ 1 {len - bl}) ; Offset to the next block
            (map~ &(filled {len - bl - &1}))
            sum~
            inc)))
      sum~)))

(defn all (n)
  (~> (range~ m n)
      (map~ filled)
      sum~
      inc))

(defn solve (cur)
  (let ([n (filled cur)])
    (if {(all cur) > 1_000_000}
        cur
        (do
          (vector-push! known-filled n)
          (solve (inc cur))))))

(solution (solve (inc m)))

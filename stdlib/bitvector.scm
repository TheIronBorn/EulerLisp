; Assuming the system supports 64-bit integers
(defn make-bitvector (len) (make-vector (inc (div len 64)) 0))
(defn bitvector-set! (bv n)
  (let ([idx (div n 64)])
  (vector-set!
    bv
    idx
    (bitwise-or
      (vector-ref bv idx)
      (<< 1 (% n 64))))))
(defn bitvector-popcount (bv)
  (~>
    (range~ 0 (dec (vector-length bv)))
    (map~ &(popcount (vector-ref bv &1)))
    sum~))

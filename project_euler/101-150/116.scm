; Solved: 22.2.2018

(def known-filled2 #(1 1 2))
(def known-filled3 #(1 1 1 2))
(def known-filled4 #(1 1 1 1 2))

; -1 => leave spot blank
; -2/3/3 => place block
(defn filled2 (len)
  (if (< len (vector-length known-filled2))
      (vector-ref known-filled2 len)
      (+ (filled2 {len - 1})
         (filled2 {len - 2}))))
(defn filled3 (len)
  (if (< len (vector-length known-filled3))
      (vector-ref known-filled3 len)
      (+ (filled3 {len - 1})
         (filled3 {len - 3}))))
(defn filled4 (len)
  (if (< len (vector-length known-filled4))
      (vector-ref known-filled4 len)
      (+ (filled4 {len - 1})
         (filled4 {len - 4}))))

(~> (range~ 3 50)
    (each~ &(vector-push! known-filled2 (filled2 &1))))
(~> (range~ 4 50)
    (each~ &(vector-push! known-filled3 (filled3 &1))))
(~> (range~ 5 50)
    (each~ &(vector-push! known-filled4 (filled4 &1))))

; dec because leaving all blocks empty does not count
(solution
  (+ (dec (vector-ref known-filled2 50))
     (dec (vector-ref known-filled3 50))
     (dec (vector-ref known-filled4 50))))

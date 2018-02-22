; Solved: 22.2.2018

; | | |
; |2|2|
;
; | | | |
; |2-2| |
; | |2-2|
; |3-3-3|
;
; | | | | |
; |2-2| | |
; |2-2|2-2|
; | |2-2| |
; | | |2-2|
; |3-3-3| |
; | |3-3-3|
; |4-4-4-4|

(def known-filled #(1 1 2 4 8))

; -1 => leave spot blank
; -2/3/4 => place block
(defn filled (len)
  (if (< len (vector-length known-filled))
      (vector-ref known-filled len)
      (+ (filled {len - 1})
         (filled {len - 2})
         (filled {len - 3})
         (filled {len - 4}))))

(~> (range~ 5 50)
    (each~ &(vector-push! known-filled (filled &1))))

(solution (vector-ref known-filled 50))

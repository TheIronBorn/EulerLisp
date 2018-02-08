; Solved: 

(defn ways-to-sum (s n)
  (cond
    [(= n 0) 0]
    [(= s 0) 0]
    [(= n 1) (if (> s 10) 0 1)]
    [else
      (~>
        (range~ 1 (min 10 s))
        (map~ &(ways-to-sum (- s &1) (dec n)))
        sum~)]))

(defn ways-to-sum2 (s n (last 10))
  (cond
    [(= s 0) 0]
    [(= n 1) (if (> s last) 0 1)]
    [else
      (~>
        (range~ 1 (min last s))
        (map~ &(ways-to-sum2 (- s &1) (dec n) &1))
        sum~)]))

; 7-1 7
; 7-2 21
; 7-3 35
; 7-4 35
; 7-5 21
; 7-6 7
; 7-7 1

(println (ways-to-sum 20 3))
(println (ways-to-sum2 2 2))

; (def w2 (* (ways-to-sum2 20 2) 7 6))
; (def w3 (* (ways-to-sum2 20 3) 7 6 5))
; (def w4 (* (ways-to-sum2 20 4) 7 6 5 4))
; (def w5 (* (ways-to-sum2 20 5) 7 6 5 4 3))
; (def w6 (* (ways-to-sum2 20 6) 7 6 5 4 3 2))
; (def w7 (* (ways-to-sum2 20 7) 7 6 5 4 3 2 1))
(def w2 (* (ways-to-sum2 20 2) 7 6))
(def w3 (* (ways-to-sum2 20 3) 7 6 5))
(def w4 (* (ways-to-sum2 20 4) 7 6 5 4))
(def w5 (* (ways-to-sum2 20 5) 7 6 5 4 3))
(def w6 (* (ways-to-sum2 20 6) 7 6 5 4 3 2))
(def w7 (* (ways-to-sum2 20 7) 7 6 5 4 3 2 1))

(def wall (+ w2 w3 w4 w5 w6 w7))
(println wall)

(def expe
  (/ (+ (* w2 2) (* w3 3)
        (* w4 4) (* w5 5)
        (* w6 6) (* w7 7))
     wall))

(println (number->float expe))

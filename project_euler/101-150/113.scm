; Solved: 28.1.2018

; All numbers 1 - 9 are decreasing & increasing
(def ways-dec #(1 1 1 1 1 1 1 1 1))
(def ways-inc #(1 1 1 1 1 1 1 1 1))

; Number of ways we can build a `n` digit
; decreasing integer starting with `last`
;
; To do so, iterate over the possible next digits (1..last)
; and sum the ways to build a decreasing number using these.
; After that, add 1, because `last 0 0 0 ...` would be possible, too
(defn ways-dec-to-sum (n last)
  (~>
    (range~ 1 last)
    (map~ &(+ (dec &1) (* (- n 2) 9)))
    (map~ &(vector-ref ways-dec &1))
    sum~
    inc))

(defn ways-inc-to-sum (n last)
  (~>
    (range~ last 9)
    (map~ &(+ (dec &1) (* (- n 2) 9)))
    (map~ &(vector-ref ways-inc &1))
    sum~))

(defn step-n (n (last 1))
  (when (<= last 9)
      (vector-push! ways-dec (ways-dec-to-sum n last))
      (vector-push! ways-inc (ways-inc-to-sum n last))
      (step-n n (inc last))))

(def n 100)
(~> (range~ 2 n)
    (map~ (fn (x) (step-n x)))
    collect)

(defn vector-sum (v)
  (~>
    (range~ 0 (dec (vector-length v)))
    (map~ &(vector-ref v &1))
    sum~))

; Of each n-digit number, 9 are both increasing and decreasing
; (11, 22, 33, ...), (111, 222, 333, ...)
(solution (+ (vector-sum ways-inc) (vector-sum ways-dec) (* n (- 9))))

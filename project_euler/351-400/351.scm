; Solved 28.1.2018
;
; The first version was correct in theory
; but would have taken years to solve the problem for high numbers.
; Rewriting it in crystal reduced the time to 15min
; and submitting the solution unlocked a long pdf
; with proofs of better algorithms.
;
; The code below is an implementation of Algorithm 6
; from the pdf. Somewhere hidden in there is some error
; (might be an integer over / underflow) so the direct result is wrong,
; but multiplying and subtracting it to get the final solution
; seems to fix that.

(def n 100_000_000)
(~> (totient-sum n) (* 6) (- (* n (inc n) 3)) solution)

; Solved: 17.12.2017

; a = fib_n, even
; b = fib_n+1
; c = fib_n+2
(defn sum_even_fib (limit (a 2) (b 3) (c 5) (sum 0))
  (if (< a limit)
    (let*
      ([a_ (+ b c)]
       [b_ (+ c a_)]
       [c_ (+ a_ b_)])
      (sum_even_fib limit a_ b_ c_ (+ sum a)))
      sum))

(solution (sum_even_fib 4000000))

; Solved: 17.12.17
; Changes to the interpreter
; * implement let syntax sugar
; * implement let* syntax sugar

(defn sum_even_fib (limit)
      (sum_even_fib_ limit 2 3 5 0))

; a = fib_n, even
; b = fib_n+1
; c = fib_n+2
(defn sum_even_fib_ (limit a b c sum)
  (if (< a limit)
    (let*
      (a_ (+ b c)
       b_ (+ c a_)
       c_ (+ a_ b_))
      (sum_even_fib_ limit a_ b_ c_ (+ sum a)))
      sum))

(println "Solution: " (sum_even_fib 4000000))

; Solved: 17.12.2017

;; $$
;; \begin{aligned}
;; f(0) =& 1 \\
;; f(1) =& 2 \\
;; f(n) =& f(n - 1) + f(n - 2)
;; \end{aligned}
;; $$
;;
;; The first few values are $1,2,3,5,8,13,21,34, \ldots$.
;;
;; It is obvious that starting from $f(2)$ every third value will be even.
;;
;; Let $a$ be some even $f(n)$, $b = f(n+1), c = f(n+2)$.
;;
;; In the next step $a \gets b + c$,
;; $b \gets b + 2c$ (the new $a$ plus $c$) and $c \gets 2b + 3c$ (the new $a$ plus the new $c$).
;;
;; The only remaining part it to start with $a = 2$, the first even $f(n)$,
;; iterate until it is greater than four million
;; and sum up all $a$ along the way.

(defn sum-even-fib (a b)
  (defn inner (a b c sum)
    (if (< a 4_000_000)
        (inner
          (+ b c)
          (+ b c c)
          (+ b b c c c)
          (+ sum a))
        sum))
  (inner a b (+ a b) 0))

(solution (sum-even-fib 2 3))

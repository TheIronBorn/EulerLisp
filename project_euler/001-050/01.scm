; Solved: 17.12.2017

;; The multiples of $x$ are $1x, 2x, 3x, \ldots$
;; so their sum is $x \cdot (1 + 2 + \ldots + \lfloor \frac{n}{x} \rfloor)$.
;;
;; To get the sum of all multiples of 3 and 5,
;; add the results of the formula above
;; and subtract the ones that were counted twice (multiples of 15).
;;
;; `(gauss-sum n)` is $1 + 2 + \ldots + n = \frac{n(n+1)}{2}$.

(def n (dec 1000))

(solution
  (+ (* 3 (gauss-sum (div n 3)))
     (* 5 (gauss-sum (div n 5)))
     (- (* 15 (gauss-sum (div n 15))))))

; Solved: 20.12.17
; Notes:
;  recursion w/ a variable for the known primes
;  takes up to much memory because environments are not freed
; Changes:
;  * add (rand low high) function
;  * add builtin (powmod base exponend mod) function
;  * add builtin bitshift

(defn nth-prime (n (cur 3))
      (if (prime? cur)
          (if (zero? n)
              cur
              (nth-prime (dec n) (+ cur 2)))
          (nth-prime n (+ cur 2))))

(println "Solution: " (nth-prime (- 10001 2)))

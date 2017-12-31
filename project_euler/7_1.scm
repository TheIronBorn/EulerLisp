; Solved: 20.12.17
; Notes:
;  recursion w/ a variable for the known primes
;  takes up to much memory because environments are not freed
; Changes:
;  * add (rand low high) function
;  * add builtin (powmod base exponend mod) function
;  * add builtin bitshift

(defn nth-prime (n) (nth-prime_ (- n 2) 3))
(defn nth-prime_ (n cur)
      (if (prime? cur)
          (if (zero? n)
              cur
              (nth-prime_ (dec n) (+ cur 2)))
          (nth-prime_ n (+ cur 2))))

(println (nth-prime 10001))

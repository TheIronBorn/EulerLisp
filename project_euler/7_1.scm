; Solved: 20.12.17
; Notes:
;  recursion w/ a variable for the known primes
;  takes up to much memory because environments are not freed
; Changes:
;  * add (rand low high) function
;  * add builtin (powmod base exponend mod) function
;  * add builtin bitshift

(def known-primes '(2 3))
(def len 2)

(defn prime? (n)
      (none-divides? n (isqrt n) 0))

(defn none-divides? (n nsq cur)
      (if (= cur len)
          #t
          (let ((p (nth cur known-primes)))
            (if (< p nsq)
              (if (divides? p n)
                  #f
                  (none-divides? n nsq (inc cur)))
              #t))))

(defn nth-prime (n) (nth-prime_ (- n 2) 3))
(defn nth-prime_ (n cur)
      (println n)
      (if (prime? cur)
          (do
            (set! known-primes (push known-primes cur))
            (set! len (inc len))
            (if (zero? n)
                cur
                (nth-prime_ (dec n) (+ cur 2))))
          (nth-prime_ n (+ cur 2))))


; (println (nth-prime 10001))
(println (nth-prime 3000))

; Solved: 20.12.17
; Notes:
;  recursion w/ a variable for the known primes
;  takes up to much memory because environments are not freed
; Changes:
;  * add (rand low high) function
;  * add builtin (powmod base exponend mod) function
;  * add builtin bitshift

; Write n as 2^r * d, d odd
(defn factor2 (n) (factor2_ n 0))
(defn factor2_ (n r)
  (if (even? n)
      (factor2_ (>> n 1) (inc r))
      (cons r n)))

; returns true if probably prime, false if not
(defn witness-loop (n r d k)
      (if (zero? k)
          #t
          (let* ((a (rand 2 (- n 2))) 
                 (x (powmod a d n)))
            (if (or (= x 1) (= x (dec n)))
                (witness-loop n r d (dec k))
                (if (subloop (dec r) x n)
                    (witness-loop n r d (dec k))
                    #f)))))

; return true to continue, false to return 'composite'
(defn subloop (times x n)
      (if (zero? times)
          #f
          (let ((x (% (* x x) n)))
               (cond
                 ((= x 1) #f)
                 ((= x (dec n)) #t)
                 (else (subloop (dec times) x n))))))

(defn nth-prime (n) (nth-prime_ (- n 3) 5))
(defn nth-prime_ (n cur)
      (println n)
      (if (prime? cur)
          (do
            ; (set! known_primes (cons cur known_primes))
            (if (zero? n)
                cur
                (nth-prime_ (dec n) (+ cur 2)))
          )
          (nth-prime_ n (+ cur 2))))


(println "Solution: " (nth-prime 10001))

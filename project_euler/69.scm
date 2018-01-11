; Solved: 11.1
; Changes: added (primes n) function,
; to get the first n primes from a hardcoded list of primes
;
; phi(n) for n = p_1^k_1 * ... * p_n^k_n (prime factors)
; is defined as
;
; n * (1 - 1/p_1) * ... * (1 - 1/p_n),
; so n / phi(n)
; gets bigger the smaller
; (1 - 1/p_1) * ... * (1 - 1/p_k) is.
;
; To get the maxium,
; maximize the number of prime factors
; and keep each prime as small as possible
;
; => solution = product of first few primes that is < 1.000.000

(defn solve (limit acc ps)
  (let ((next (* acc (fst ps))))
    (if (> next limit)
        acc
        (solve limit next (rst ps)))))

(println "Solution: " (solve 1000000 1 (primes 100)))

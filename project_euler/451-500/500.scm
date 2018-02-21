; Solved: 21.2.2018

; Example:
;  2^1 * 3^1 * 5^1 * 7^1 has 2^4 factors,
;  but 2^2 < 5 and 2^3 has 4 factors (instead of 2, like 2^1)
;  so the smallest number would be 2^3 * 3^1 * 5^1
;
; Start with a min-heap of primes
; pop the first element 500500 times,
; pushing its square back on the stack

(def n 500500)
(def modulus 500500507)

(def candidates
  (cons
    (cons 2 2)
    (~> (step~ 3 2)
        (select~ prime?)
        (map~ &(cons &1 &1))
        (take~ n))))

(def pq (make-min-priority-queue candidates))

(defn solve (needed (acc 1))
  (if (zero? needed)
      acc
      (let ([best (priority-queue-pop! pq)])
        (priority-queue-insert!
          pq
          (% (* (fst best) (fst best)) modulus)
          (* (rst best) (rst best)))
        (solve
          (dec needed)
          (% (* acc (fst best)) modulus)))))

(solution (solve n))

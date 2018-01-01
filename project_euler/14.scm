; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)
; Improved to 3:10 by skipping a step for odd numbers
; since 3n + 1 for odd n is always even
;
; TODO: Solve in < 60s

(defn collatz-length (n) (helper n 1))
(defn helper (n len)
  (if (= n 1)
    len
    (if (even? n)
      (helper (>> n 1) (inc len))
      (helper (+ (* 3 (>> n 1)) 2) (+ len 2)))))

; We can be pretty sure that the number with the max length will be odd
(defn rsolve (from to max-n max-len)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (if (> cur-len max-len)
              (rsolve (+ from 2) to from cur-len)
              (rsolve (+ from 2) to max-n max-len)))))

; (rsolve 1 1000000 1 1)
(rsolve 1 50000 1 1)

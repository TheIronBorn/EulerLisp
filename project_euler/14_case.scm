; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)
; * Improved to 3:10 by skipping a step for odd numbers
;   since 3n + 1 for odd n is always even
;
; * Improve to 2:30 by skipping multiple steps
;   switching on n % 4
; * Improve to 2:20 by using func refs instead of rc lambdas
; * Improve to 1:40 by removing some `.clone()`s
; TODO: Solve in < 60s

(defn collatz-length (n) (helper n 1))
(defn helper (n len)
  (case n
        (1 len)
        (2 (inc len))
        (else
          (case (% n 4)
            (0 (helper (>> n 2) (+ len 2)))
            (1 (helper (+ (* 3 (>> n 2)) 1) (+ len 3)))
            (2 (helper (+ (* 3 (>> n 2)) 2) (+ len 3)))
            (3 (helper (+ (* 9 (>> n 2)) 8) (+ len 4)))
          ))))

; We can be pretty sure that the number with the max length will be odd
(defn rsolve (from to max-n max-len)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (if (> cur-len max-len)
              (rsolve (+ from 2) to from cur-len)
              (rsolve (+ from 2) to max-n max-len)))))

(rsolve 1 1000000 1 1)

; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)

; * Improved to 3:10 by skipping a step for odd numbers
;   since 3n + 1 for odd n is always even
;
; * Improve to 2:30 by skipping multiple steps
;   switching on n % 4
; TODO: Solve in < 60s

(defn collatz-length (n) (helper n 1))
(defn helper (n len)
  (case n
        (1 len)
        (2 (inc len))
        (4 (+ len 2))
        (else
          (case (% n 8)
            (0 (helper
                 (>> n 3)
                 (+ len 3)))
            (1 (helper
                 (+ (* 9 (>> n 3)) 2)
                 (+ len 5)))
            (2 (helper
                 (+ (* 3 (>> n 3)) 1)
                 (+ len 4)))
            (3 (helper
                 (+ (* 9 (>> n 3)) 4)
                 (+ len 5)))
            (4 (helper
                 (+ (* 3 (>> n 3)) 2)
                 (+ len 4)))
            (5 (helper
                 (+ (* 3 (>> n 3)) 2)
                 (+ len 4)))
            (6 (helper
                 (+ (* 9 (>> n 3)) 8)
                 (+ len 5)))
            (7 (helper
                 (+ (* 27 (>> n 3)) 26)
                 (+ len 6)))
            ))))

; We can be pretty sure that the number with the max length will be odd
(defn rsolve (from to max-n max-len)
      (println from)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (if (> cur-len max-len)
              (rsolve (+ from 2) to from cur-len)
              (rsolve (+ from 2) to max-n max-len)))))
; 15.71

(rsolve 1 100000 1 1)


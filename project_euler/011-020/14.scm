; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)
; * Improved to 3:10 by skipping a step for odd numbers
;   since 3n + 1 for odd n is always even
;
; * Improve to 2:30 by skipping multiple steps
;   switching on n % 4
; * Improve to 2:20 by using func refs instead of rc lambdas
; * Improve to 1:40 by removing some `.clone()`s
; * Improve to 20s via new `vector-ref`
(def known #(0 1 2))
(def known-len 2)

(defn collatz-length (n) (helper n 1))
(defn helper (n len)
  (if (<= n known-len)
      (+ len (vector-ref known n))
      (case (% n 4)
        (0 (helper (>> n 2) (+ len 2)))
        (1 (helper (+ (* 3 (>> n 2)) 1) (+ len 3)))
        (2 (helper (+ (* 3 (>> n 2)) 2) (+ len 3)))
        (3 (helper (+ (* 9 (>> n 2)) 8) (+ len 4))))))

; We can be pretty sure that the number with the max length will be odd
(defn rsolve (from to max-n max-len)
      (println from)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (vector-push! known cur-len)
            (set! known-len (inc known-len))
            (if (> cur-len max-len)
              (rsolve (inc from) to from cur-len)
              (rsolve (inc from) to max-n max-len)))))

(rsolve 3 1000000 1 1)

; (def known (make-vector 1000000 0))
(def known #(1 2))

; Solved: 30.12.17
; (defn helper (n len)
;       (if (<= n (length known))
;           (+ len (nth (dec n) known))
;           (if (even? n)
;             (helper (>> n 1) (inc len))
;             (helper (inc (* n 3)) (inc len)))))
(defn helper (n len)
      (if (<= n (length known))
          (+ len (nth (dec n) known))
          (if (even? n)
            (helper (>> n 1) (inc len))
            (helper (inc (* n 3)) (inc len)))))

(defn collatz-length (n) (helper n 1))

(def max-n 1)
(def max-len 1)

(defn solve (from to)
      (println from)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (vector-push! known cur-len)
            (if (> cur-len max-len)
                (do
                  (set! max-n from)
                  (set! max-len cur-len)))
            (solve (+ from 2) to))))

(solve 3 10000)

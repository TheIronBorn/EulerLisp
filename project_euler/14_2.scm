; Solved 31.12,
; but finding the solution takes 4:35 (more than a minute)
; TODO: Solve in < 60s

(defn helper (n len)
  (if (= n 1)
    len
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
            (if (> cur-len max-len)
                (do
                  (set! max-n from)
                  (set! max-len cur-len)))
            (solve (+ from 2) to))))

(defn rsolve (from to max-n max-len)
      (println from)
      (if (> from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (if (> cur-len max-len)
              (rsolve (+ from 2) to from cur-len)
              (rsolve (+ from 2) to max-n max-len)))))

(rsolve 1 1000000 1 1)

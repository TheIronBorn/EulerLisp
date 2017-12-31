; Unsolved
(def known '())

(defn helper (n len)
      (if (<= n (length known))
          (+ len (nth (dec n) known))
          (cond
            ((= n 1) len)
            ((even? n) (helper (>> n 1) (inc len)))
            (else (helper (inc (* n 3)) (inc len))))))

(defn collatz-length (n)
      (helper n 1))

(def max-n 1)
(def max-len 1)

(defn solve (from to)
      (println from)
      (if (= from to)
          (println max-n)
          (let ((cur-len (collatz-length from)))
            (set! known (push known cur-len))
            (if (> cur-len max-len)
                (do
                  (set! max-n from)
                  (set! max-len cur-len)))
            (solve (inc from) to))))

(solve 1 100000)

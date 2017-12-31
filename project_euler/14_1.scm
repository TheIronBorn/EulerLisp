(def known (make-vector 1000000 0))

; Solved: 30.12.17
(defn collatz (n)
  (if (and (< n 1000000) (> (nth n known) 0))
      (nth n known)
      (let ((res
            (cond
              ((= n 1) 0)
              ((even? n) (collatz (>> n 1)))
              (else (collatz (inc (* n 3)))))))
        (vector-set! known n (inc res))
        (inc res))))

(def max-n 1)
(def max-len 1)

(defn solve (from to)
      (println from)
      (if (= from to)
          (println max-n)
          (let ((cur-len (collatz from)))
            (if (> cur-len max-len)
                (do
                  (set! max-n from)
                  (set! max-len cur-len)))
            (solve (inc from) to))))

(solve 1 1000)

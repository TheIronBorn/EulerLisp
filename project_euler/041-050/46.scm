; Solved 8.1

(defn goldbach? (n)
      (goldbach?_ n 1))

(defn goldbach?_ (n sq)
  (cond
    (< n (* 2 sq sq)) #f
    (prime? (- n (* 2 sq sq))) #t
    else (goldbach?_ n (inc sq))))


(defn loop (cur)
  (if (or (prime? cur) (goldbach? cur))
      (loop (+ cur 2))
      (println cur)))

(loop 9)

; Times:
;
;  19.1.18: 4.77s

(defn fib (n)
      (if (<= n 1)
          n
          (+ (fib (- n 1))
             (fib (- n 2)))))

(println (fib 30))

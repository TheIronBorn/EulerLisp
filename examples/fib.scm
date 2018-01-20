; Times:
;
;  19.1.18: 4.77s
;  20.1.18: 3.94s, switch envs to activation frames

(defn fib (n)
      (if (<= n 1)
          n
          (+ (fib (- n 1))
             (fib (- n 2)))))

(println (fib 30))

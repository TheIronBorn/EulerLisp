; Solved 15.1.18

(defn coin-sums (n coins)
  (cond
    (zero? n) 1
    (nil? coins) 0
    (> (fst coins) n) 0
    else
    (reduce-sum
      (fn (x) (coin-sums
                (- n (* x (fst coins)))
                (rst coins)))
      (range 0 (div n (fst coins))))))

(def ps (primes 100))

; This is not 100% right,
; it might fail if the result is a prime
; and its coin sum is 5001
(defn solve (cur)
  (let (cs (coin-sums cur ps))
    (println "cur: " cur " -> " cs)
    (if (> cs 5000)
      cur
      (solve (inc cur)))))

(println "Solution: " (solve 2))

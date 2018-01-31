; Solved 15.1.18

(defn coin-sums (n coins)
      (cond
        [(zero? n) 1]
        [(nil? coins) 0]
        [(> (fst coins) n) 0]
        [else
          (reduce-sum
            &(coin-sums
               (- n (* &1 (fst coins)))
               (rst coins))
            (range 0 (div n (fst coins))))]))

(def ps (primes 100))

; This is not 100% right,
; it might fail if the result is a prime
; and its coin sum is 5001
(~>
  (step~ 2)
  (select~ &(> (coin-sums &1 ps) 5000))
  first~
  (println "Solution: "))

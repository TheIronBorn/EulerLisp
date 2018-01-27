; Solved 25.1.18
; Time: 6:51
; Solution: 303963552391

; phi(n) = number of divisors of n with gcd(d, n) = 1
(defn phi (n)
  (* n (reduce-product &(- 1 (/ 1 (fst &1))) (prime-factors n))))

(~>
  (range~ 2 1000000)
  (map~ &(phi &1))
  sum~
  (println "Solution: "))

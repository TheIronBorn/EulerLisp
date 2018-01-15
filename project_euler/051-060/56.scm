; Solved 11.1
; Changes: make print & println variadic

(defn digit-sum (n)
  (~> n bignum-digits sum))

(defn loop (a b max-ds)
  (println "a = " a)
  (cond
    (>= b 100) (loop (inc a) 1 max-ds)
    (>= a 100) max-ds
    else (loop a (inc b)
               (max max-ds
                    (digit-sum (bigpow (bignum a) b))))))

(println "Solution: " (loop 1 1 0))

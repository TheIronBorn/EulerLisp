; Solved 11.1
; Changes: make print & println variadic

(defn digit-sum (n) (~> n bignum-digits sum))

(println "Solution: "
  (reduce-max
    (fn (a)
        (reduce-max
          &(digit-sum (bigpow (bignum a) &1))
          0 (range 1 99)))
    0 (range 1 99)))

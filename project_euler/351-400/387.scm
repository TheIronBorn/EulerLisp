; Solved: 27.1.18

(defn digit-sum (n) (sum (number->digits n)))
(defn harshad? (n) (divides? (digit-sum n) n))
(defn strong-harshad? (n) (prime? (div n (digit-sum n))))

(defn extend-harshad (hs)
     (flatmap
       (fn (h1)
           (~> (range~ 0 9)
               (map~ &(+ &1 (* 10 h1)))
               (select~ harshad?)
               collect))
       hs))

(defn extend-prime (hs)
     (flatmap
       (fn (h1)
           (~> (range~ 0 9)
               (map~ &(+ &1 (* 10 h1)))
               (select~ prime?)
               collect))
       hs))


(defn sum-harshad-primes (arr)
    (~> arr (select strong-harshad?) extend-prime sum))

(defn solve (remaining basis (acc 0))
  (if (zero? remaining)
      acc
      (solve
        (dec remaining)
        (extend-harshad basis)
        (+ acc (sum-harshad-primes basis)))))

; Start w/ 2-digit numbers,
; a 1-digit number divided by the sum of its digits it 1 -> not prime
(~> (range 1 9) extend-harshad (solve 12) solution)

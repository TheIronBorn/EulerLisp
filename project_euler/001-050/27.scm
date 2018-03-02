; Solved 1.1.2018

(defn count-cycles (n a b)
      (if (prime? (+ (* n n) (* a n) b))
          (count-cycles (inc n) a b)
          (dec n)))

(defn solve (a b)
  (defn inner (a b max-a max-b max-len)
      (if (>= a 1000)
          (inner -999 (inc b) max-a max-b max-len)
          (if (> b 1000)
              (* max-a max-b)
              (let ([len (count-cycles 1 a b)])
                (if (> len max-len)
                    (inner (inc a) b a b len)
                    (inner (inc a) b max-a max-b max-len))))))
  (inner a b 0 0 0))

(solution (solve -999 -1000))

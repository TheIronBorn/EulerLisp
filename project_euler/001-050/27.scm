; Solved 1.1

(defn count-cycles (n a b)
      (if (prime? (+ (* n n) (* a n) b))
          (count-cycles (inc n) a b)
          (dec n)))

(defn solve (a b (maxa 0) (maxb 0) (maxlen 0))
      (if (>= a 1000)
          (solve -999 (inc b) maxa maxb maxlen)
          (if (> b 1000)
              (* maxa maxb)
              (let ([len (count-cycles 1 a b)])
                (if (> len maxlen)
                    (solve (inc a) b a b len)
                    (solve (inc a) b maxa maxb maxlen))))))

(println "Solution: " (solve -999 -1000))

; Solved 1.1

(defn count-cycles (n a b)
      (if (prime? (+ (* n n) (* a n) b))
          (count-cycles (inc n) a b)
          (dec n)))

(defn solve (a b maxa maxb maxlen)
      (if (>= a 1000)
          (solve -999 (inc b) maxa maxb maxlen)
          (if (> b 1000)
              (list maxa maxb)
              (let ((len (count-cycles 1 a b)))
                (if (> len maxlen)
                    (solve (inc a) b a b len)
                    (solve (inc a) b maxa maxb maxlen))))))

(println "Solution: " (solve -999 -1000 0 0 0))

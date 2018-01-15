; Solved 1.1

(defn factor-sum (n)
      (- (sum (factors n)) n))

(defn amicable? (n)
   (let (fsum (factor-sum n))
        (and (!= n 1)
             (!= n fsum)
             (= n (factor-sum fsum)))))

(defn solve ((from 1) (acc 0))
      (if (> from 10000)
        acc
        (solve
          (inc from)
          (if (amicable? from)
            (+ acc from)
            acc))))

(println "Solution: " (solve))

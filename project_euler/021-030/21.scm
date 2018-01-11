; Solved 1.1

(defn factor-sum (n)
      (- (sum (factors n)) n))

(defn amicable? (n)
   (let ((fsum (factor-sum n)))
        (and (!= n 1)
             (!= n fsum)
             (= n (factor-sum fsum)))))

(defn solve (from to acc)
      (if (> from to)
        acc
        (solve
          (inc from)
          to
          (if (amicable? from)
            (+ acc from)
            acc))))

(println "Solution: " (solve 1 10000 0))

; Solved 1.1.2018

(defn factor-sum (n)
      (- (sum (factors n)) n))

(defn amicable? (n)
   (let ([fsum (factor-sum n)])
        (and (!= n 1)
             (!= n fsum)
             (= n (factor-sum fsum)))))

(~> (range~ 1 10000) (select~ amicable?) sum~ solution)

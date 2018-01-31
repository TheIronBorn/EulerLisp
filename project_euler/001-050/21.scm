; Solved 1.1
; Changes:
;  * switch to streams

(defn factor-sum (n)
      (- (sum (factors n)) n))

(defn amicable? (n)
   (let ([fsum (factor-sum n)])
        (and (!= n 1)
             (!= n fsum)
             (= n (factor-sum fsum)))))

(~>
  (range~ 1 10000)
  (select~ amicable?)
  sum~
  (println "Solution: "))

; Solved 22.1.18

(defn count-powers (of (cur 1) (acc 0))
    (let (len (bignum-num-digits (bigpow of cur)))
      (cond
        (= len cur) (count-powers of (inc cur) (inc acc))
        else acc)))

(~>
  (range 1 9)
  (map bignum)
  (map count-powers)
  sum
  (println "Solution: "))

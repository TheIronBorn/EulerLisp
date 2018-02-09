; Solved 22.1.2018

(defn count-powers (of (cur 1) (acc 0))
    (let ([len (number-of-digits (pow of cur))])
      (if (= len cur)
          (count-powers of (inc cur) (inc acc))
          acc)))

(~> (range 1 9) (reduce-sum count-powers) solution)

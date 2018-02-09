; Solved: 18.1.2018

(defn number-reverse (n)
  (~> n number->digits reverse digits->bignum))

(defn lychrel? (cur (n 50))
  (if (zero? n)
      #t
      (let ([cur-rev (number-reverse cur)])
        (if (= cur cur-rev)
            #f
            (lychrel? (+ cur cur-rev) (dec n))))))

(~> (range~ 1 9999) (count~ lychrel?) solution)

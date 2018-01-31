; Solved: 18.1.18

(defn number-reverse (n)
  (~> n
      bignum-digits
      reverse
      digits->bignum))

(defn lychrel? (cur (n 50))
  (if (zero? n)
      #t
      (let* ([cur-rev (number-reverse cur)]
             [sum_ (bignum+ cur cur-rev)])
        (if (palindromic? (bignum-digits sum_))
            #f
            (lychrel? sum_ (dec n))))))

(~>
  (range~ 1 9999)
  (map~ bignum)
  (count~ lychrel?)
  (println "Solution: "))

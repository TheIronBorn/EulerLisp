; Solved: 18.1.18

(defn number-reverse (n)
  (~> n
      bignum-digits
      reverse
      digits->bignum))

(defn palindromic? (n)
      (let (ds (bignum-digits n))
        (= ds (reverse ds))))

(defn lychrel? (cur (n 50))
  (if (zero? n)
      #t
      (let* (cur-rev (number-reverse cur)
             sum_ (bignum+ cur cur-rev))
        (if (palindromic? sum_)
            #f
            (lychrel? sum_ (dec n))))))

(defn solve (cur (count 0))
  (println "cur = " cur)
  (cond
    (>= cur 10000) count
    (lychrel? (bignum cur)) (solve (inc cur) (inc count))
    else (solve (inc cur) count)))

(println "Solution: " (solve 1))

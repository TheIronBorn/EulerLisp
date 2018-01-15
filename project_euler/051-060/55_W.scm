; TODO: cur must be a bignum,
; otherwise it would overflow

(defn number-reverse (n)
  (~> n
      digits
      reverse
      digits->number))

(defn palindromic? (n)
      (let (ds (digits n))
        (= ds (reverse ds))))

(defn lychrel (cur n)
  (if (zero? n)
      #t
      (let* (cur-rev (number-reverse cur)
             sum_ (bignum+ cur cur-rev))
        (println sum_)
        (if (palindromic? sum_)
            #f
            (lychrel sum_ (dec n))))))

(println (lychrel (bignum 47) 50))
(println (lychrel (bignum 349) 50))
(println (lychrel (bignum 196) 50))

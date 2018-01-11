(defn resize (n)
  (~> n
      bignum-chunks
      (take 10)
      bignum-from-chunks))

(def big1 (bigpow (bignum 2) 17401))
(def big2 (bigpow (resize big1) 5))
(def big3 (bigpow (resize big2) 9))
(def big4 (bignum* big3 (bignum 28433)))

(println 
  (~>
    big4
    bignum-digits
    reverse
    (take 10)
    (apply str)
    ))

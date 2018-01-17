; Solved 11.1

(defn resize (n)
  (~> n
      bignum-chunks
      (take 2)
      bignum-from-chunks))

(defn resizing-bigpow (b e)
  (cond
    (= e 0) (bignum 1)
    (even? e) (resizing-bigpow (resize (bignum* b b)) (div e 2))
    else (bignum* b (resizing-bigpow b (dec e)))))

(println 
  (~>
    (resizing-bigpow (bignum 2) 7830457)
    (bignum* (bignum 28433))
    bignum-digits
    (take 10)
    digits->number
    inc))

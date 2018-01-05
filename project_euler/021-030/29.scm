; Solved 5.1
; Changes: Add bignum multiplication

(def candidates (range 2 100))
(def result
     (flatmap 
       (fn (a) (map (fn (b) (bigpow (bignum a) b)) candidates))
       candidates))

(defn dedup (arr)
      (dedup_ arr -1 '()))

(defn dedup_ (arr last acc)
  (cond
    ((nil? arr) acc)
    ((= last (fst arr)) (dedup_ (rst arr) last acc))
    (else 
      (dedup_ (rst arr) (fst arr) (cons (fst arr) acc))
      )))

(~> result sort dedup length println)

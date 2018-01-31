; Solved: 27.1.18

(defn digit-sum (n)
      (sum (number->digits n)))

(defn harshad? (n)
      (divides? (digit-sum n) n))


(defn extend-harshad (hs)
     (flatmap
       (fn (h1)
           (~> (range~ 0 9)
               (map~ &(+ &1 (* 10 h1)))
               (select~ harshad?)
               collect))
       hs))

(defn extend-prime (hs)
     (flatmap
       (fn (h1)
           (~> (range~ 0 9)
               (map~ &(+ &1 (* 10 h1)))
               (select~ prime?)
               collect))
       hs))

(defn strong-harshad? (n) (prime? (div n (digit-sum n))))

(def harshad1 (range 1 9))

(def rt-harshad2 (extend-harshad harshad1))
(def rt-harshad3 (extend-harshad rt-harshad2))
(def rt-harshad4 (extend-harshad rt-harshad3))
(def rt-harshad5 (extend-harshad rt-harshad4))
(def rt-harshad6 (extend-harshad rt-harshad5))
(def rt-harshad7 (extend-harshad rt-harshad6))
(def rt-harshad8 (extend-harshad rt-harshad7))
(def rt-harshad9 (extend-harshad rt-harshad8))
(def rt-harshad10 (extend-harshad rt-harshad9))
(def rt-harshad11 (extend-harshad rt-harshad10))
(def rt-harshad12 (extend-harshad rt-harshad11))
(def rt-harshad13 (extend-harshad rt-harshad12))

; A 1-digit number divided by the sum of its digits it 1 -> not prime
(def srt-harshad2 (select strong-harshad? rt-harshad2))
(def srt-harshad3 (select strong-harshad? rt-harshad3))
(def srt-harshad4 (select strong-harshad? rt-harshad4))
(def srt-harshad5 (select strong-harshad? rt-harshad5))
(def srt-harshad6 (select strong-harshad? rt-harshad6))
(def srt-harshad7 (select strong-harshad? rt-harshad7))
(def srt-harshad8 (select strong-harshad? rt-harshad8))
(def srt-harshad9 (select strong-harshad? rt-harshad9))
(def srt-harshad10 (select strong-harshad? rt-harshad10))
(def srt-harshad11 (select strong-harshad? rt-harshad11))
(def srt-harshad12 (select strong-harshad? rt-harshad12))
(def srt-harshad13 (select strong-harshad? rt-harshad13))

(def harshad-prime3 (extend-prime srt-harshad2))
(def harshad-prime4 (extend-prime srt-harshad3))
(def harshad-prime5 (extend-prime srt-harshad4))
(def harshad-prime6 (extend-prime srt-harshad5))
(def harshad-prime7 (extend-prime srt-harshad6))
(def harshad-prime8 (extend-prime srt-harshad7))
(def harshad-prime9 (extend-prime srt-harshad8))
(def harshad-prime10 (extend-prime srt-harshad9))
(def harshad-prime11 (extend-prime srt-harshad10))
(def harshad-prime12 (extend-prime srt-harshad11))
(def harshad-prime13 (extend-prime srt-harshad12))
(def harshad-prime14 (extend-prime srt-harshad13))

(def all
     (list 
       harshad-prime3
       harshad-prime4
       harshad-prime5
       harshad-prime6
       harshad-prime7
       harshad-prime8
       harshad-prime9
       harshad-prime10
       harshad-prime11
       harshad-prime12
       harshad-prime13))

(defn reduce-bignum-sum (arr)
      (if (nil? arr)
          (bignum 0)
          (reduce (fn (x acc)
                      (bignum+ (bignum x) acc))
                  (bignum (fst arr))
                  (rst arr))))

(println (reduce bignum+ (bignum 0)
           (list
             (reduce-bignum-sum harshad-prime3)
             (reduce-bignum-sum harshad-prime4)
             (reduce-bignum-sum harshad-prime5)
             (reduce-bignum-sum harshad-prime6)
             (reduce-bignum-sum harshad-prime7)
             (reduce-bignum-sum harshad-prime8)
             (reduce-bignum-sum harshad-prime9)
             (reduce-bignum-sum harshad-prime10)
             (reduce-bignum-sum harshad-prime11)
             (reduce-bignum-sum harshad-prime12)
             (reduce-bignum-sum harshad-prime13)
             (reduce-bignum-sum harshad-prime14)
             )))

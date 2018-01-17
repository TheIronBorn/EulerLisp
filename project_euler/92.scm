; Solved 15.1.18
; Time: 4:31,
; improved to 1.67 by using the fact
; that all permutations of a number map to the same number
;
; Solution: 8581146

(defn digit-squares (n (acc 0))
  (if (zero? n)
      acc
      (digit-squares (div n 10)
                     (+ acc (square (% n 10))))))

(defn trace (n)
  (cond
    (= n 0) #f
    (= n 1) #f
    (= n 89) #t
    else (trace (digit-squares n))))

(def lookup (map trace (range 0 567)))

(defn generate (size missing)
  (if (= 1 missing)
      (list (list size))
      (flatmap
        (fn (x)
          (map (fn (rest) (cons x rest))
            (generate (- size x) (dec missing))))
        (range 0 size)
      )
    ))

(def perms (generate 7 10))

(def base (fac 7))
(defn number-of-permutations (dcs)
  (div base (reduce-product fac dcs)))

(defn perm-digit-squares (perm (d 0) (acc 0))
      (if (nil? perm)
          acc
          (perm-digit-squares (rst perm)
                              (inc d)
                              (+ acc (* (fst perm) d d)))))

(defn solve (i to (acc 0))
      (if (= i to)
          acc
          (let*
            (perm (list-ref perms i)
             res (perm-digit-squares perm))
            (if (list-ref lookup res)
                (solve (inc i)
                       to
                       (+ acc (number-of-permutations perm)))
                (solve (inc i) to acc)))))

(println "Solution: " (solve 0 (length perms)))

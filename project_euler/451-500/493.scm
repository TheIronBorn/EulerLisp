; Solved: 14.2.2018

; The normal version (using factorials) would not work for (binomial 70 20)
; because bignum division is not implemented yet
(defn binomial_ (n k)
  (cond
    [{k > n} 0]
    [{k = 0} 1]
    [else
     (/ (* (binomial_ (dec n) (dec k)) n) k)]))

; Ways to sum d integers to create n,
; not restricted to some order
(defn partitions (n d)
  (cond
    [{d > n} (list)]
    [{d = 1} (list (list n))]
    [else
     (flatmap
       (fn (a)
         (map (fn (ps) (cons a ps))
              (partitions {n - a} {d - 1})))
       (range 1 (min 10 (- n (- d 1)))))]))

; Precalculate binomials to improve program speed
(def binomial10
  (list->vector (map (fn (k) (binomial_ 10 k)) (range 0 20))))
(def binomial7
  (list->vector (map (fn (k) (binomial_ 7 k)) (range 0 7))))

(defn solve (colors)
  (~>
    (partitions 20 colors)
    (reduce-sum
      (fn (partition)
        (~> 
          partition
          (reduce (fn (cur acc) {acc * (vector-ref binomial10 cur)}) 1)
          (* (vector-ref binomial7 colors)))))))

; Number of ways to select 20 of 70 balls
(def all (number->float (binomial_ 70 20)))

; Having only one color is not possible
(~>
  (range~ 2 7)
  (map~ &(/ (* &1 (solve &1)) all))
  sum~
  str
  string->chars
  (take {2 + 9})
  chars->string
  solution)

; Solved: 29.1.18
; Time: 3:19

(def n 10000000)

; (defn phi (n)
;   (reduce (fn (pf acc) (- acc (div acc (fst pf))))
;           n
;           (prime-factors n)))

(defn permutation? (a)
      (let ([ds1 (number->digits (fst a))]
            [ds2 (number->digits (rst a))])
        (and
          (= (length ds1) (length ds2))
          (= (sort ds1) (sort ds2)))))

(def tots (filled-list (inc n) 0))
(defn fill (cur)
  (when (<= cur n)
    (set-nth! tots cur cur)
    (fill (inc cur))))
(defn update (p k n)
  (when (<= k n)
    (let ([old (list-ref tots k)])
      (set-nth! tots k (- old (div old p))))
    (update p (+ k p) n)))
(defn loop (p acc)
  (if (<= p n)
      (do
        (if (= p (list-ref tots p)) (update p p n))
          (loop
            (inc p)
            (+ acc (- p (list-ref tots p)))))
    acc))
(fill 0)
(loop 2 0)

 (~>
   (range~ 2 n)
   (map~ &(cons &1 (list-ref tots &1)))
   (select~ permutation?)
   collect
   (min-by (fn (c) (/ (fst c) (rst c))))
   (println "Solution: "))

; Solved 8.1

(def myprimes 
   (~> (range~ 1000 9999)
       (select~ prime?)
       (map~ number->digits)
       collect))

(defn find-permutations_ (sds b max-b acc)
  (if (>= b max-b)
      acc
      (let* ([elem (list-ref myprimes b)]
             [sds_ (sort elem)])
        (if (= sds sds_)
            (find-permutations_ sds (inc b) max-b (cons elem acc))
            (find-permutations_ sds (inc b) max-b acc)))))

(defn find-permutations (a)
  (let ([elem (list-ref myprimes a)])
    (find-permutations_
      (sort elem)
      (inc a)
      (length myprimes)
      (list elem))))

(defn subsequences (seq len)
  (cond
    [(zero? len) (list)]
    [(nil? seq) (list)]
    [(= len 1) (map list seq)]
    [else
      (append
        (subsequences (rst seq) len)
        (let ([rests (subsequences (rst seq) (dec len))])
             (if (nil? rests)
                 '()
                 (map &(cons (fst seq) &1) rests))))]))

(defn solve (from to (acc '()))
  (if (>= from to)
      acc
      (let ([perm (find-permutations from)])
        (if (>= (length perm) 3)
          (solve (inc from) to (cons (map digits->number perm) acc))
          (solve (inc from) to acc)))))

(def sequences (solve 0 (length myprimes)))
(def all-sequences (flatmap (fn (x) (subsequences x 3)) sequences))

(defn ascending?_ (seq diff)
  (let ([a (fst seq)]
        [b (rst seq)])
    (cond
      [(nil? b) #t]
      [(= diff (- (fst b) a)) (ascending?_ b diff)]
      [else #f])))

(defn ascending? (seq)
      (ascending?_ (rst seq) (- (frst seq) (fst seq))))

(println
  "Solutions: "
  (map &(apply str &1)
       (~> all-sequences (map sort) (select ascending?))))

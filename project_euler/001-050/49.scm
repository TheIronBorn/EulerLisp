; Solved 8.1.2018

(def myprimes 
   (~> (range~ 1000 9999)
       (select~ prime?)
       (map~ number->digits)
       collect
       list->vector))

(defn find-permutations_ (sds b max-b acc)
  (if (>= b max-b)
      acc
      (let* ([elem (vector-ref myprimes b)]
             [sds_ (sort elem)])
        (if (= sds sds_)
            (find-permutations_ sds (inc b) max-b (cons elem acc))
            (find-permutations_ sds (inc b) max-b acc)))))

(defn find-permutations (a)
  (let ([elem (vector-ref myprimes a)])
    (find-permutations_
      (sort elem)
      (inc a)
      (vector-length myprimes)
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

(defn solve (from to)
  (defn inner (cur acc)
    (if (>= cur to)
        acc
        (let ([perm (find-permutations cur)])
          (if (>= (length perm) 3)
            (inner (inc cur) (cons (map digits->number perm) acc))
            (inner (inc cur) acc)))))
  (inner from '()))

(def sequences (solve 0 (vector-length myprimes)))
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

(solution
  (map &(apply str &1)
       (~> all-sequences (map sort) (select ascending?))))

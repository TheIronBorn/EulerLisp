; Solved 8.1
; Changes: Add bignum multiplication

(def myprimes (~>
                (range 1000 9999)
                (select prime?)
                (map digits)))

(defn find-permutations (a)
  (let (elem (list-ref myprimes a))
    (find-permutations_
      (sort elem)
      (inc a)
      (length myprimes)
      (list elem))))

(defn find-permutations_ (sds b max-b acc)
  (if (>= b max-b)
      acc
      (let* (elem (list-ref myprimes b)
             sds_ (sort elem))
        (if (= sds sds_)
            (find-permutations_ sds (inc b) max-b (cons elem acc))
            (find-permutations_ sds (inc b) max-b acc)))))

(defn subsequences (seq len)
  (cond
    (zero? len) (list)
    (nil? seq) (list)
    (= len 1) (map list seq)
    else
      (append
        (subsequences (rst seq) len)
        (map
          (fn (x) (cons (fst seq) x))
          (subsequences (rst seq) (dec len))))))

(defn solve (from to acc)
  (if (>= from to)
      acc
      (let (perm (find-permutations from))
        (if (>= (length perm) 3)
          (solve (inc from) to (cons (map digits->number perm) acc))
          (solve (inc from) to acc)
          ))))

(def sequences (solve 0 (length myprimes) '()))
(def all-sequences (flatmap (fn (x) (subsequences x 3)) sequences))

(defn ascending? (seq)
      (ascending?_ (rst seq) (- (frst seq) (fst seq))))

(defn ascending?_ (seq diff)
  (let (a (fst seq)
        b (rst seq))
    (cond
      (nil? b) #t
      (= diff (- (fst b) a)) (ascending?_ b diff)
      else #f)))

(println
  "Solutions: "
  (map (fn (x) (apply str x))
       (~> all-sequences (map sort) (select ascending?))))

; Solved 8.1
; Changes: Add bignum multiplication

(defn digits (n) (digits_ n '()))
(defn digits_ (n acc)
  (if (= n 0)
      acc
      (let ((digit (% n 10))
            (n (/ n 10)))
        (digits_ n (cons digit acc)))))

(defn digits->number (ds)
  (string->number (apply str ds)))

(def primes #())

(defn loop (from)
      (if (< from 9999)
          (do
            (if (prime? from)
                (vector-push! primes (digits from)))
            (loop (inc from)))))
(loop 1000)

(defn find-permutations (a)
  (let ((elem (vector-ref primes a)))
    (find-permutations_
      (sort elem)
      (inc a)
      (length primes)
      (list elem))))

(defn find-permutations_ (sds b max-b acc)
  (if (>= b max-b)
      (reverse acc)
      (let* ((elem (vector-ref primes b))
             (sds_ (sort elem)))
        (if (= sds sds_)
            (find-permutations_ sds (inc b) max-b (cons elem acc))
            (find-permutations_ sds (inc b) max-b acc)))))

(defn subsequences (seq len)
  (cond
    ((zero? len) (list))
    ((nil? seq) (list))
    ((= len 1) (map list seq))
    (else
      (append
        (subsequences (rst seq) len)
        (map
          (fn (x) (cons (fst seq) x))
          (subsequences (rst seq) (dec len)))))))

(defn solve (from to acc)
  (if (>= from to)
      acc
      (let ((perm (find-permutations from)))
        (if (>= (length perm) 3)
          (solve (inc from) to (cons (map digits->number perm) acc))
          (solve (inc from) to acc)
          ))))

(def sequences (solve 0 (length primes) '()))
(def all-sequences (flatmap (fn (x) (subsequences x 3)) sequences))

(defn ascending? (seq)
      (ascending?_ (rst seq) (- (frst seq) (fst seq))))

(defn ascending?_ (seq diff)
  (let ((a (fst seq))
        (b (rst seq)))
    (cond
      ((nil? b) #t)
      ((= diff (- (fst b) a))
       (ascending?_ b diff))
      (else #f))))

(println (map (fn (x) (apply str x)) (select ascending? all-sequences)))

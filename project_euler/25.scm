; Solved: 17.12.17

(defn fib (n a b)
      (if (= 0 n)
          a
          (fib (dec n) b (+ a b))))

(defn for (from to fun)
      (if (<= from to)
        (do (fun from)
            (for (inc from) to fun))))


(def base 1000000000)

; We can assume that b is always bigger,
; this simplifies some steps
(defn list+ (a b) (list+_ a b 0 '()))
(defn list+_ (a b carry acc)
      (cond
        ((nil? a)
         (if (zero? carry)
           (if (nil? b) acc (append acc b))
           (if (nil? b)
               (push acc carry)
               (let* ((sum (+ (fst b) carry)))
                 (if (> sum base)
                     (list+_ '() (rst b) 1 (push acc (- sum base)))
                     (list+_ '() (rst b) 0 (push acc sum)))))))
        (else
          (let* ((sum (+ (+ (fst a) (fst b)) carry)))
            (if (> sum base)
                (list+_ (rst a) (rst b) 1 (push acc (- sum base)))
                (list+_ (rst a) (rst b) 0 (push acc sum)))))))

(def a '(1))
(def b '(1))

(defn numlength (n)
  (cond
    ((< n 10) 1) 
    ((< n 100) 2) 
    ((< n 1000) 3) 
    ((< n 10000) 4) 
    ((< n 100000) 5) 
    ((< n 1000000) 6) 
    ((< n 10000000) 7) 
    ((< n 100000000) 8) 
    ((< n 1000000000) 9)))

(defn lastlength (l)
      (let ((last (nth (dec (length l)) l)))
        (numlength last)))

(defn listlength (l)
  (+ (lastlength l)
     (* 9 (dec (length l)))))

(defn solve (n)
      (if (= (listlength a) 1000)
          (println n)
          (do
             (let ((new_a b) (new_b (list+ a b)))
                  (set! a new_a)
                  (set! b new_b))
             (solve (inc n)))))

(solve 1)

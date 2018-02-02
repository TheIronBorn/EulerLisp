; Solved: 1.2.2018

(defn next-row (row)
  (push
    (cons 1
      (map &(+ (fst &1) (frst &1))
           (zip row (rst row))))
    1))

(defn rows (n cur (acc '()))
  (if (zero? n)
      acc
      (rows (dec n)
            (next-row cur)
            (append acc cur))))

(defn squarefree? (n)
  (let ([pf (prime-factors n)])
    (or (empty? pf)
        (all? &(= 1 (rst &1)) (prime-factors n)))))

(~> (rows 51 '(1))
    sort
    uniq
    (select squarefree?)
    sum
    solution)

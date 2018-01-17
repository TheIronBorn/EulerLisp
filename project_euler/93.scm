(defn brackets (dgts ops)
  (let (a (fst dgts)
        b (frst dgts)
        c (frrst dgts)
        d (frrrst dgts)
        op1 (fst ops)
        op2 (frst ops)
        op3 (frrst ops))
    (list
      (eval (list op2
            (list op1 a b)
            (list op3 c d)))
      (eval (list op3
        (list
          op2
          (list op1 a b)
          c)
        d))
      (eval (list op3
        (list op1
              a
              (list op2 b c))
        d))
      (eval (list op1 a
            (list op3
                  (list op2 b c)
                  d)))
      (eval (list op1 a
            (list op2
                  b
                  (list op3 c d))))
      )))

(defn div_ (a b)
  (if (= b 0)
      -99999
      (/ a b)))

(def op-combs (combinations 3 (list '+ '- '* 'div_)))

(def abcds
     (flatmap
       (fn (a)
           (flatmap
             (fn (b)
                 (flatmap
                   (fn (c)
                       (map
                         (fn (d) (list a b c d))
                         (range 1 (dec c))))
                   (range 1 (dec b))))
             (range 1 (dec a))))
       (range 1 9)))

(defn streak (results (acc 0) (last 0))
      (cond
        (nil? results) acc
        (<= (fst results) 0)
          (streak (rst results) acc last)
        (= (dec (fst results)) last)
          (streak (rst results) (inc acc) (fst results))
        else
          acc))

(defn results (abcd)
     (~>
       (flatmap
         (fn (dgs)
           (~>
             (flatmap
               (fn (ops)
                   (map (fn (x)
                            (if (integer? x) x -999))
                   (brackets dgs ops)))
               op-combs)
             sort
             uniq))
         (permutations abcd))
       sort
       uniq
       reverse
       streak
     ))

(defn max-by (fun lst)
  (if (nil? lst)
      '()
      (reduce (fn (x acc)
                  (let (fx (fun x))
                       (if (> fx (rst acc))
                           (cons x fx)
                           acc)))
              (cons
                (fst lst)
                (fun (fst lst)))
              (rst lst))))

(~>
  (max-by results abcds)
  fst
  reverse
  (apply str)
  (println "Solution: "))

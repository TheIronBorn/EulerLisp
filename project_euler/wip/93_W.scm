; Solved: 29.1.18

; Special "broken" division as an easy way to handle
; stuff like (/ a 0).
; Returns a number that could not possibly appear in any useful sequence
(defn div_ (a b) (if (= b 0) -99999 (/ a b)))
(defn calc (op a b)
  (case op
    ['+ (+ a b)]
    ['- (- a b)]
    ['* (* a b)]
    ['/ (div_ a b)]))

(defn brackets (dgts ops)
  (let ([a (fst dgts)]
        [b (frst dgts)]
        [c (frrst dgts)]
        [d (frrrst dgts)]
        [op1 (fst ops)]
        [op2 (frst ops)]
        [op3 (frrst ops)])
    (list
      (calc op2 (calc op1 a b) (calc op3 c d))
      (calc op3 (calc op2 (calc op1 a b) c) d)
      (calc op3 (calc op1 a (calc op2 b c)) d)
      (calc op1 a (calc op3 (calc op2 b c) d))
      (calc op1 a (calc op2 b (calc op3 c d))))))

(def op-combs (combinations 3 (list '+ '- '* '/)))

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
                   (range 2 (dec b))))
             (range 3 (dec a))))
       (range 4 9)))

(defn streak (results (acc 0) (last 0))
      (cond
        [(nil? results) acc]
        [(<= (fst results) 0)
         (streak (rst results) acc last)]
        [(= (dec (fst results)) last)
         (streak (rst results) (inc acc) (fst results))]
        [else acc]))

(defn results (abcd)
     (~>
       (flatmap
         (fn (dgs)
           (~>
             (flatmap
               (fn (ops)
                   (map &(if (integer? &1) &1 -999)
                        (brackets dgs ops)))
               op-combs)
             sort uniq))
         (permutations abcd))
       sort uniq streak))

(~>
  (max-by results abcds)
  reverse
  (apply str)
  (println "Solution: "))

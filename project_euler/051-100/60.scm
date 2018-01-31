; Solved: 27.1.18
; Time: 45.77s

(defn concat (a b)
  (+ b
     (* (pow 10 (floor (log10 b)))
        10 a)))

(defn concat-prime? (a b)
      (and
        (prime? (concat a b))
        (prime? (concat b a))))

; I had to increase this step by step until there was a solution
(def ps (primes 1100))
(def maxl (dec (length ps)))

(def families5
     (flatmap (fn (ai) (let ([a (list-ref ps ai)])
       (flatmap (fn (bi) (let ([b (list-ref ps bi)])
         (flatmap (fn (ci) (let ([c (list-ref ps ci)])
           (flatmap (fn (di) (let ([d (list-ref ps di)])
                      (map
                        (fn (ei) (list a b c d (list-ref ps ei)))
                        (~> (range~ di maxl)
                            (select~ &(let ([v (list-ref ps &1)])
                                        (and (concat-prime? a v) (concat-prime? b v)
                                             (concat-prime? c v) (concat-prime? d v))))
                            collect))))
                    (~> (range~ ci maxl)
                        (select~ &(let ([v (list-ref ps &1)])
                                    (and (concat-prime? a v) (concat-prime? b v)
                                         (concat-prime? c v))))
                        collect))))
                  (~> (range~ bi maxl)
                      (select~ &(let ([v (list-ref ps &1)])
                                  (and (concat-prime? a v) (concat-prime? b v))))
                      collect))))
                (~> (range~ ai maxl)
                    (select~ &(concat-prime? a (list-ref ps &1)))
                    collect))))
              (range 0 maxl)))

(~>
  families5
  fst
  sum
  (println "Solution: "))

; Solved: 27.1.2018

(defn concat (a b)
  (+ b
     (* (pow 10 (floor (log10 b)))
        10 a)))

(defn concat-prime? (a b)
      (and
        (prime? (concat a b))
        (prime? (concat b a))))

; I had to increase this step by step until there was a solution
(def ps (list->vector (primes 1100)))
(def maxl (dec (length ps)))

(def families5
     (flatmap (fn (ai) (let ([a (vector-ref ps ai)])
       (flatmap (fn (bi) (let ([b (vector-ref ps bi)])
         (flatmap (fn (ci) (let ([c (vector-ref ps ci)])
           (flatmap (fn (di) (let ([d (vector-ref ps di)])
                      (map
                        (fn (ei) (list a b c d (vector-ref ps ei)))
                        (~> (range~ di maxl)
                            (select~ &(let ([v (vector-ref ps &1)])
                                        (and (concat-prime? a v) (concat-prime? b v)
                                             (concat-prime? c v) (concat-prime? d v))))
                            collect))))
                    (~> (range~ ci maxl)
                        (select~ &(let ([v (vector-ref ps &1)])
                                    (and (concat-prime? a v) (concat-prime? b v)
                                         (concat-prime? c v))))
                        collect))))
                  (~> (range~ bi maxl)
                      (select~ &(let ([v (vector-ref ps &1)])
                                  (and (concat-prime? a v) (concat-prime? b v))))
                      collect))))
                (~> (range~ ai maxl)
                    (select~ &(concat-prime? a (vector-ref ps &1)))
                    collect))))
              (range 0 maxl)))

(~> families5 fst sum solution)

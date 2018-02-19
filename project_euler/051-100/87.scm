; Solved: 19.2.2018

(def max-n 50_000_000)

(def ps
  (cons 2
    (~>
      (range~ 3 (isqrt max-n) 2)
      (select~ prime?)
      collect)))

(def p2 (map square ps))

(def p3
  (~>
    ps
    (map &(pow &1 3))
    (select &(<= &1 max-n))))

(def p4
  (~>
    p2
    (map square)
    (select &(<= &1 max-n))))

(def bv (make-bitvector max-n))

(each
  (fn (a)
    (each
      (fn (b)
        (~> (map &(+ a b &1) p2)
            (select &(< &1 max-n))
            (each &(bitvector-set! bv &1))))
     p3))
  p4)

(solution (bitvector-popcount bv))

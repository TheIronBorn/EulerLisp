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

; Assuming the system supports 64-bit integers
(def bitarray (make-vector (inc (div max-n 64)) 0))
(defn insert! (n)
  (let ([idx (div n 64)])
  (vector-set!
    bitarray
    idx
    (bitwise-or
      (vector-ref bitarray idx)
      (<< 1 (% n 64))))))

(each
  (fn (a)
    (each (fn (b)
        (~>
          p2
          (map &(+ a b &1))
          (select &(< &1 max-n))
          (each insert!)))
     p3))
  p4)

(~>
  (range~ 0 (dec (vector-length bitarray)))
  (map~ &(popcount (vector-ref bitarray &1)))
  sum~
  solution)

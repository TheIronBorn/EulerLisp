; Solved: 20.2.2018

; The only repunits we need to care about are
; 111_b = x, 1111_b = x, ... with b so that 11_b < max_n,
; because x is 11_{x - 1}

(def max-n (pow 10 12))

(defn collect-repunits_ (base cur rest (acc '()))
  (let ([next (+ cur rest)])
    (if (<= next max-n)
      (collect-repunits_
        base
        (* base cur)
        next
        (cons next acc))
      acc)))

; Sorting a list w/ > 1_000_000 elements leads to a stack overflow,
; as a quick fix, split the repunits into buckets
(def buckets (make-vector 10_000 '()))

(defn collect-repunits (base)
  (~>
    (collect-repunits_ base (* base base) (+ base 1))
    (each
      (fn (rp)
        (let ([idx (% rp 1000)])
          (vector-set! buckets idx
                       (cons rp (vector-ref buckets idx))))))))

; Find the maximal base where 111_b <= n
(defn find-max (n (cur 2))
  (if (> (+ 1 (* cur (+ 1 cur))) n)
      (dec cur)
      (find-max n (inc cur))))

(~>
  max-n
  find-max
  (range~ 2)
  (each~ collect-repunits))

(~>
  (range~ 0 999)
  (map~ (fn (a) (~> (vector-ref buckets a) sort uniq sum)))
  sum~
  inc
  solution)

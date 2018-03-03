; Solved: 19.2.2018
; Time: 39:20
;
; TODO: Look at https://en.wikipedia.org/wiki/Legendre%27s_formula
; TODO: Is it viable to create a vector with lists for each number?

(defn count-p (p n (acc 1))
  (if (divides? p n)
      (count-p p (div n p) (inc acc))
      acc))

; find the first m so that m!
; contains the prime factor p
; at least k times
(defn find-m (p k (cur 1))
  (if {k <= 1}
      (* p cur)
      (find-m
        p
        (- k (count-p p (inc cur)))
        (inc cur))))

(defn find-m-all (pfs)
  (reduce-max (fn (pf) (find-m (fst pf) (rst pf))) 1 pfs))

(~>
  ; (range~ 1 100_000_000)
  (range~ 2 100_000_000)
  (map~ prime-factors)
  (map~ find-m-all)
  sum~
  solution)

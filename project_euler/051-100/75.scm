; Solved: 6.2.2018

; Using euclids formula,
; generate all primitive pythagorean triples
; (the ones that are not multiples of other triples)
; and collect their sums
(defn primitives-n (m)
  (~>
    (range~ (if (even? m) 1 2) m 2)
    (select~ &(= 1 (gcd &1 m)))
    (map~ (fn (n)
              (+
                (- (square m) (square n))
                (* 2 m n)
                (+ (square m) (square n)))))
    collect))

(defn primitives (max-len)
  (~>
    (range~ 2 (isqrt max-len))
    (flatmap-list~ primitives-n)
    (select~ &(<= &1 max-len))
    collect))

(def max-len 1_500_000)
(def all (primitives max-len))

; Each multiple of a triple (with a max. length of max-len)
; gives us one possibility to create a triangle with this length.
; Iterate over all triples and for each multiply,
; increase the entry in sieve for (a + b + c) * factor
; by one

(def sieve (filled-list max-len 0))

(defn update-multiples (base cur)
  (when (< cur max-len)
    (set-nth! sieve cur (inc (list-ref sieve cur)))
    (update-multiples base (+ cur base))))

(defn process-item (item) (update-multiples item (dec item)))
(each process-item all)

(solution (count &(= 1 &1) sieve))

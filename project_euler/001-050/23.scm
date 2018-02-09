; Solved 5.1.2018

; The Problem descriptions contains a different limit,
; but the one below is more accurate
(def max-n 20161)

(defn factor-sum (n) (- (sum (factors n)) n))
(defn abundant? (n) (> (factor-sum n) n))

(def abundant-sums (filled-list max-n #f))
(defn abundant-sum? (n) (list-ref abundant-sums (dec n)))

(def abundants
  (~> (range~ 12 (inc max-n))
      (select~ abundant?)
      collect))

(def len (dec (length abundants)))
(def init (list-ref abundants 0))

(push! abundants 0)

(defn loop2 (pa pb va vb limit)
      (if (< pb len)
        (if (or (>= pa len)
                (>= va limit))
          (let ([next-vb (list-ref abundants (inc pb))])
            (loop2 0 (inc pb)
                   init next-vb
                   (- max-n next-vb)))
          (do
            (set-nth! abundant-sums (dec (+ va vb)) #t)
            (loop2 (inc pa) pb
                   (list-ref abundants (inc pa)) vb
                   limit)))))
(loop2 0 0 init init (- max-n init))

(~> (range~ 1 max-n)
    (select~ (fn (x) (not (abundant-sum? x))))
    sum~
    solution)

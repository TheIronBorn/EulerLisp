; Solved 5.1

; The Problem descriptions contains a different limit,
; but the one below is more accurate
(def max-n 20161)

(defn factor-sum (n) (- (sum (factors n)) n))
(defn abundant? (n) (> (factor-sum n) n))

(def abundants '())

(def abundant-sums (filled-list max-n #f))
(defn abundant-sum? (n) (list-ref abundant-sums (dec n)))

(defn loop (from to)
      (if (<= from to)
        (do
          (if (abundant? from)
              (push! abundants from))
          (loop (inc from) to))))

(loop 12 max-n)
(push! abundants 0)

(def len (dec (length abundants)))
(def init (list-ref abundants 0))

(defn loop2 (pa pb va vb limit)
      (if (< pb len)
        (if (or (>= pa len)
                (>= va limit))
          (let (next-vb (list-ref abundants (inc pb)))
            (loop2 0 (inc pb)
                   init next-vb
                   (- max-n next-vb)))
          (do
            (set-nth! abundant-sums (dec (+ va vb)) #t)
            (loop2 (inc pa) pb
                   (list-ref abundants (inc pa)) vb
                   limit)))))

(defn loop3 (from to acc)
      (if (> from to)
          acc
          (loop3 (inc from)
                 to
                 (if (abundant-sum? from)
                     acc
                     (+ acc from)))))
(println "Solution: " (loop3 1 max-n 0))

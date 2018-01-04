; Solved 2.1

; (def max-n 28123)
(def max-n 20161)
; (def max-n 1000)
(defn factor-sum (n) (- (sum (factors n)) n))
(defn abundant? (n) (> (factor-sum n) n))

(def abundants #())

(def abundant-sums (make-vector max-n #f))
(defn abundant-sum? (n) (nth (dec n) abundant-sum))

(defn loop (from to)
      (if (<= from to)
        (do
          (if (abundant? from)
              (vector-push! abundants from))
          (loop (inc from) to))))

(loop 12 max-n)
(vector-push! abundants 0)

(def len (dec (length abundants)))
(def init (nth 0 abundants))

(println len)

(defn loop2 (pa pb va vb)
      (println pb)
      (if (>= pb len)
        #t
        (if (or 
              (>= pa len)
              (> (+ va vb) 10000))
          (loop2 0 (inc pb)
                 init
                 (nth (inc pb) abundants))
          (do
            (vector-set! abundant-sums (dec (+ va vb)) #t)
            (loop2 (inc pa) pb
                   (nth (inc pa) abundants) vb)
            ))))

(println (loop2 0 0 init init))

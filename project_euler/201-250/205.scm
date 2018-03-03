; Solved: 14.2.2018

(defn total (n sides (acc 0))
      (if (zero? n)
          acc
          (total (dec n)
                 sides
                 (+ acc (rand 1 sides)))))

(defn pete-wins? ()
  (let ([a (total 9 4)]
        [b (total 6 6)])
    (cond
      [(< a b) #f]
      [(> a b) #t]
      [else (pete-wins?)])))

(def colin-res (make-vector 37 0))

(defn vector-inc! (id n)
  (vector-set! id n (inc (vector-ref id n))))

(~> (combinations~ 6 (list 1 2 3 4 5 6))
    (map~ list-sum)
    (each~ &(vector-inc! colin-res &1)))

(defn count-wins (pete)
  (~>
    (range~ 0 (dec pete))
    (map~ &(vector-ref colin-res &1))
    sum~))

(def total-wins
  (~>
    (combinations~ 9 (list 1 2 3 4))
    (map~ list-sum)
    (map~ &(count-wins &1))
    sum~))

(~>
  (/ (number->float total-wins) (pow 6 6) (pow 4 9))
  solution)

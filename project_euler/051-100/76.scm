; Solved 15.1.2018

(def results #(1))

(defn index (n limit)
  (dec (+ limit (div (* n (dec n)) 2))))

(defn partition-sum (n limit (acc 0))
  (if (= limit 1)
      (inc acc)
      (partition-sum n
                     (dec limit)
                     (+ acc
                        (vector-ref results
                                  (index (- n limit)
                                         (min (- n limit) limit)))))))

(defn first-partitions (n limit)
  (cond
    [(<= n 1) n]
    [(= n limit) (inc (partition-sum n (dec limit)))]
    [else (partition-sum n limit)]))

(defn inner-loop (n (limit 1))
  (if (<= limit n)
    (let ([val (first-partitions n limit)])
         (vector-push! results val)
         (inner-loop n (inc limit)))))

(defn loop (from (acc 0))
  (if (<= from 100)
    (do
      (inner-loop from)
      (~>
        (vector-length results)
        dec
        (vector-ref results)
        (loop (inc from))))
    acc))

(solution (dec (loop 2)))

; Solved 15.1.18

(def results '(1))
(def len 1)

(defn partition-sum (n limit (acc 0))
  (if (= limit 1)
      (inc acc)
      (partition-sum n
                     (dec limit)
                     (+ acc
                        (list-ref results
                                  (index (- n limit)
                                         (min (- n limit) limit)))))))

(defn index (n limit)
  (dec (+ limit (/ (* n (dec n)) 2))))

(defn first-partitions (n limit)
  (cond
    (<= n 1)
      n
    (= n limit) 
      (inc (partition-sum n (dec limit)))
    else
      (partition-sum n limit)))

(defn loop (from (acc 0))
  (if (<= from 100)
    (do
      (inner-loop from)
      (let (last (list-ref results (dec len)))
        (loop (inc from) last)))
    acc))

(defn inner-loop (n (limit 1))
  (if (<= limit n)
    (let (val (first-partitions n limit))
         (push! results val)
         (set! len (inc len))
         (inner-loop n (inc limit)))))

(println "Solution: " (dec (loop 2)))

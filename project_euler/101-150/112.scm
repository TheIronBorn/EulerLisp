; Solved: 26.1.2018

(defn decreasing? (n (last -1))
  (if (zero? n)
      #t
      (if (>= (% n 10) last)
          (decreasing? (div n 10) (% n 10))
          #f)))
(defn increasing? (n (last 10))
  (if (zero? n)
      #t
      (if (<= (% n 10) last)
          (increasing? (div n 10) (% n 10))
          #f)))

(defn bouncy? (n)
      (and (not (increasing? n))
           (not (decreasing? n))))

(defn solve (cur (acc 0))
  (cond
    [(= (* acc 100) (* 99 cur)) cur]
    [(bouncy? (inc cur)) (solve (inc cur) (inc acc))]
    [else (solve (inc cur) acc)]))

(solution (solve 1))

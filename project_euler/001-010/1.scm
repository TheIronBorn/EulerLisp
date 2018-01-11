; Solved: 17.12.17
(defn multiple_of? (x n) (= 0 (% x n)))

(defn sum_to_ (n i sum)
  (if (< i n)
      (if (or (multiple_of? i 3) (multiple_of? i 5))
          (sum_to_ n (inc i) (+ sum i))
          (sum_to_ n (inc i) sum))
      sum))
(defn sum_to (n) (sum_to_ n 1 0))

(println "Solution: " (sum_to 1000))

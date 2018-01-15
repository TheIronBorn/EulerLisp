; Solved: 17.12.17
(defn solve ((i 1) (sum 0))
      (if (>= i 1000)
        sum
        (solve (inc i)
               (if (or (divides? 3 i) (divides? 5 i))
                 (+ sum i)
                 sum))))

(println "Solution: " (solve))

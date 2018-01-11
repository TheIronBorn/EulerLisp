; Solved: 22.12.17

(defn prime-sum (limit)
  (defn helper (limit cur acc)
        (if (> cur limit)
            acc
            (if (prime? cur)
                (helper limit (+ cur 2) (+ acc cur))
                (helper limit (+ cur 2) acc))))
  (helper limit 3 2))

(println "Solution: " (prime-sum 2000000))

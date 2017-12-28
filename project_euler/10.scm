; Solved: 22.12.17

(defn prime-sum (limit) (prime-sum_ limit 3 2))
(defn prime-sum_ (limit cur acc)
      (if (> cur limit)
          acc
          (if (prime? cur)
              (prime-sum_ limit (+ cur 2) (+ acc cur))
              (prime-sum_ limit (+ cur 2) acc))))

(println (prime-sum 2000000))

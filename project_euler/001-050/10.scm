; Solved: 22.12.2017

(defn prime-sum (limit (cur 3) (acc 2))
      (if (> cur limit)
          acc
          (if (prime? cur)
              (prime-sum limit (+ cur 2) (+ acc cur))
              (prime-sum limit (+ cur 2) acc))))

(solution (prime-sum 2000000))

; Solved: 22.12.2017

(defn prime-sum (limit)
  ; Alternate steps of 2 and 4
  ; to skip over multiples of 3
  (defn inner (cur acc s1 s2)
      (if (> cur limit)
          acc
          (if (prime? cur)
              (inner (+ cur s1) (+ acc cur) s2 s1)
              (inner (+ cur s1) acc s2 s1))))
  (inner 7 (+ 2 3 5) 4 2))

(solution (prime-sum 2_000_000))

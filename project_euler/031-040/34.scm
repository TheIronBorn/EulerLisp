; Solved 6.1

(def facs '())
(map (fn (x) (push! facs (fac x))) (range 0 9))

(defn digit-sum (n (acc 0))
  (if (= n 0)
      acc
      (digit-sum (/ n 10)
               (+ acc (list-ref facs (% n 10))))))

(def max-n (* 7 (fac 9)))

(defn digit-fac? (n)
      (= n (digit-sum n)))

(defn solve (cur (acc 0))
  (if (<= cur max-n)
      (if (digit-fac? cur)
          (solve (inc cur) (+ acc cur))
          (solve (inc cur) acc))
      acc))

(println "Solution: " (solve 3))

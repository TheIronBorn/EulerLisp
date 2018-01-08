; Solved 6.1

(def facs #())
(map (fn (x) (vector-push! facs (fac x))) (range 0 9))

(defn digits (n) (digits_ n 0))
(defn digits_ (n acc)
  (if (= n 0)
      acc
      (digits_ (/ n 10)
               (+ acc (vector-ref facs (% n 10))))))

(def max-n (* 7 (fac 9)))


(defn digit-fac? (n)
      (= n (digits n)))

(defn solve (cur acc)
  (if (<= cur max-n)
      (if (digit-fac? cur)
          (solve (inc cur) (+ acc cur))
          (solve (inc cur) acc))
      acc))

(println (solve 3 0))

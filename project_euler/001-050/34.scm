; Solved 6.1.2018

(def facs
  (~>
    (range 0 9)
    (map fac)
    list->vector))

(defn digit-sum (n (acc 0))
  (if (= n 0)
      acc
      (digit-sum
        (div n 10)
        (+ acc (vector-ref facs (% n 10))))))

(defn digit-fac? (n) (= n (digit-sum n)))

(~>
  (range~ 3 (* 7 (fac 9)))
  (select~ digit-fac?)
  sum~
  solution)

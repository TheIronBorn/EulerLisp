; Solved 6.1.2018

(def facs
  (~> (range 0 9)
      (map fac)
      list->vector))

(defn digit-sum (n)
  (defn inner (n acc)
    (if (= n 0)
        acc
        (inner
          (div n 10)
          (+ acc (vector-ref facs (% n 10))))))
  (inner n 0))

(defn digit-fac? (n) (= n (digit-sum n)))

(~>
  (range~ 3 (* 7 (fac 9)))
  (select~ digit-fac?)
  sum~
  solution)

; Solved 6.1.2018

(def facs (map fac (range 0 9)))

(defn digit-sum (n (acc 0))
  (if (= n 0)
      acc
      (digit-sum
        (div n 10)
        (+ acc (list-ref facs (% n 10))))))

(defn digit-fac? (n) (= n (digit-sum n)))

(~>
  (range~ 3 (* 7 (fac 9)))
  (select~ digit-fac?)
  sum~
  solution)

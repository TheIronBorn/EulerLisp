; Solved: 28.1.2018

(defn r-max (a)
  (~> (range~ 1 (if (odd? a) (* 2 a) a) 2)
      (map~ &(% (* 2 &1 a) (* a a)))
      (reduce-max~ id 0)))

(~> (range~ 3 1000)
    (map~ &(max (r-max &1) 2))
    sum~
    solution)

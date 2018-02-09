; Solved 22.1.2018

(defn find-cur (d) (div (dec (* 3 d)) 7))

(~>
  (range~ 1 1000000)
  (select~ &(!= &1 7))
  (map~ &(/ (find-cur &1) &1))
  (reduce~ max 0)
  numerator
  solution)

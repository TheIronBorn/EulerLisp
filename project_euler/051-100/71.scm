; Solved 22.1.18,
; changed on 31.1.18 to remove wrong assumptions

(defn find-cur (d) (div (dec (* 3 d)) 7))
(~>
  (range~ 1 1000000)
  (select~ &(!= &1 7))
  (map~ &(/ (find-cur &1) &1))
  (reduce~ max 0)
  numerator
  (println "Solution: "))
